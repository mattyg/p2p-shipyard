package com.plugin.holochainforegroundserviceconsumer

import android.app.Activity
import android.content.Intent
import android.content.Context
import android.content.ServiceConnection
import android.content.ComponentName
import android.os.IBinder
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.JSArray
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import android.app.NotificationChannel
import android.app.NotificationManager
import android.util.Log
import android.webkit.WebView
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.delay
import java.io.File
import android.net.Uri;
import com.plugin.holochainforegroundservice.IHolochainService
import com.plugin.holochainforegroundservice.InstallAppRequestAidl
import com.plugin.holochainforegroundservice.SignZomeCallRequestAidl
import android.os.SharedMemory
import java.nio.ByteBuffer

@TauriPlugin
class HolochainConsumerPlugin(private val activity: Activity): Plugin(activity) {
    private var mService: IHolochainService? = null
    private lateinit var webView: WebView
    private lateinit var injectHolochainClientEnvJavascript: String
    private val LOG_TAG = "HolochainConsumerPlugin"
    private var isBinding = false

    // IPC Connection to HolochainService using AIDL
    private val mConnection = object : ServiceConnection {
        override fun onServiceConnected(className: ComponentName, service: IBinder) {
            mService = IHolochainService.Stub.asInterface(service)
            Log.d(LOG_TAG, "IHolochainService connected")
            isBinding = false
        }

        override fun onServiceDisconnected(className: ComponentName) {
            mService = null
            Log.d(LOG_TAG, "IHolochainService disconnected")
            isBinding = false
        }
    }

    /// Load the plugin, start the service
    override fun load(webView: WebView) {
        super.load(webView)
        this.webView = webView

        // Load holochain client injected javascript from resource file
        val resourceInputStream = this.activity.resources.openRawResource(R.raw.injectholochainclientenv)
        this.injectHolochainClientEnvJavascript = resourceInputStream.bufferedReader().use { it.readText() }
    
        // Bind to HolochainService provided by other app
        bindInternal()
    }

    /// Call the mobile-conductor-admin to install a happ into conductor
    @Command
    fun installApp(invoke: Invoke) {
        val args = invoke.parseArgs(InstallAppRequestArgs::class.java)
        
        // Bind to running service
        this.bindInternal();
        
        // Write appBundleBytes to shared memory
        // This is necessary because AIDL IPC calls have a 1MB limit
        val appBundleSharedMemory = SharedMemory.create(args.appId, args.appBundleBytes.size)
        val appBundleSharedMemoryBuffer: ByteBuffer = appBundleSharedMemory.mapReadWrite()
        appBundleSharedMemoryBuffer.put(args.appBundleBytes)

        // Install app into conductor service
        this.mService?.installApp(InstallAppRequestAidl(
            args.appId,
            appBundleSharedMemory,
            args.membraneProofs,
            args.agent,
            args.networkSeed
        ))

        // Clear the shared memory
        SharedMemory.unmap(appBundleSharedMemoryBuffer)
        appBundleSharedMemory.close()

        invoke.resolve();
    }

    // Call the mobile-conductor-admin to get an authorized app web socket,
    //  then inject the magic config into the webview.
    @Command
    fun appWebsocketAuth(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdRequestArgs::class.java)
        
        // Bind to running service
        this.bindInternal();

        // Create app websocket with authentication token
        val res = this.mService?.appWebsocketAuth(args.appId)

        // Inject launcher env into web view
        this.injectHolochainClientEnv(args.appId, res!!.port, res!!.token)      

        // Return app websocket auth data
        val obj = res!!.toJSObject()
        invoke.resolve(obj)      
    }

    private fun injectHolochainClientEnv(appId: String, appWebsocketPort: Int, appWebsocketToken: UByteArray) {
        // Declare js helper function for injecting holochain client env, bundled with dependencies
        this.webView.evaluateJavascript(this.injectHolochainClientEnvJavascript, null)

        // Inject holochain client env
        val tokenJsArray = appWebsocketToken.toMutableList().toJSArray();

        this.webView.evaluateJavascript(
            """injectHolochainClientEnv("${appId}", ${appWebsocketPort}, ${tokenJsArray});""", 
            null
        )
    }

    /// Call the mobile-conductor-admin to sign a zome call
    @Command
    fun signZomeCall(invoke: Invoke) {
        val args = invoke.parseArgs(SignZomeCallRequestArgs::class.java)
        val res = this.mService?.signZomeCall(SignZomeCallRequestAidl(
            args.provenance,
            args.cellIdDnaHash,
            args.cellIdAgentPubKey,
            args.zomeName,
            args.fnName,
            args.capSecret,
            args.payload,
            args.nonce,
            args.expiresAt,
        ))
        invoke.resolve(res!!.toJSObject()) 
    }

    /// Connect to already running service
    private fun bindInternal() {
        if(this.mService == null && this.isBinding == false) {
            this.isBinding = true

            val intent = Intent()
            intent.setComponent(ComponentName("com.holochainapps.mobile_conductor_admin", "com.plugin.holochainforegroundservice.HolochainService"))
            activity.bindService(intent, this.mConnection, 8)
        }
    }
}
