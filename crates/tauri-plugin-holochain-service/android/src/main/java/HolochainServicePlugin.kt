package com.plugin.holochain_service

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
import com.plugin.holochain_service.toJSArray
import java.io.File
import android.net.Uri
import android.os.SharedMemory
import java.nio.ByteBuffer

@TauriPlugin
class HolochainServicePlugin(private val activity: Activity): Plugin(activity) {
    private var mService: IHolochainService? = null
    private lateinit var webView: WebView
    private lateinit var injectHolochainClientEnvJavascript: String
    private val LOG_TAG = "HolochainServicePlugin"

    // IPC Connection to HolochainService using AIDL
    private val mConnection = object : ServiceConnection {
        override fun onServiceConnected(className: ComponentName, service: IBinder) {
            mService = IHolochainService.Stub.asInterface(service)
            Log.d(LOG_TAG, "IHolochainService connected")
        }

        override fun onServiceDisconnected(className: ComponentName) {
            mService = null
            Log.d(LOG_TAG, "IHolochainService disconnected")
        }
    }

    /// Load the plugin, start the service
    override fun load(webView: WebView) {
        super.load(webView)
        this.webView = webView

        // Load holochain client injected javascript from resource file
        val resourceInputStream = this.activity.resources.openRawResource(R.raw.injectholochainclientenv)
        this.injectHolochainClientEnvJavascript = resourceInputStream.bufferedReader().use { it.readText() }

        // Start the service
        runBlocking {
            launchInternal()
        }
    }

    /// Start the service
    /// - Starts the foreground service
    /// - Launches a conductor
    /// - Creates an admin websocket
    @Command
    fun launch(invoke: Invoke) {
        val args = invoke.parseArgs(HolochainArgs::class.java)
        launchInternal()
        invoke.resolve()
    }
    
    /// Stop the service
    @Command
    fun shutdown(invoke: Invoke) {
        this.mService?.shutdown()
        invoke.resolve()
    }

    /// Get the holochain conductor admin websocket port
    @Command
    fun getAdminPort(invoke: Invoke) {
        val res: Int? = this.mService?.getAdminPort()
        val obj = JSObject()
        obj.put("port", res)
        invoke.resolve(obj)
    }

    /// Install a happ into conductor
    @Command
    fun installApp(invoke: Invoke) {
        val args = invoke.parseArgs(InstallAppRequestArgs::class.java)

        // Write appBundleBytes to shared memory
        val appBundleSharedMemory = SharedMemory.create(args.appId, args.appBundleBytes.size)
        val appBundleSharedMemoryBuffer: ByteBuffer = appBundleSharedMemory.mapReadWrite()
        appBundleSharedMemoryBuffer.put(args.appBundleBytes)

        // Call installApp on sevice
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

        invoke.resolve()
    }

    /// Is an app with the given app_id installed
    @Command
    fun isAppInstalled(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdRequestArgs::class.java)
        val res = this.mService?.isAppInstalled(args.appId)

        val obj = JSObject()
        obj.put("installed", res)
        invoke.resolve(obj)
    }

    /// Uninstall an installed app
    @Command
    fun uninstallApp(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdRequestArgs::class.java)
        this.mService?.uninstallApp(args.appId)
        invoke.resolve()
    }

    /// Enable an installed app
    @Command
    fun enableApp(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdRequestArgs::class.java)
        this.mService?.enableApp(args.appId)
        invoke.resolve()
    }

    /// Disable an installed app
    @Command
    fun disableApp(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdRequestArgs::class.java)
        this.mService?.disableApp(args.appId)
        invoke.resolve()
    }

    /// List installed happs in conductor
    @Command
    fun listInstalledApps(invoke: Invoke) {
        val res = this.mService?.listInstalledApps()
        val obj = JSObject() 
        obj.put("installedApps", res!!.toJSArray())
        invoke.resolve(obj)
    }

    /// Get or create an app websocket with authentication token
    @Command
    fun appWebsocketAuth(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdRequestArgs::class.java)
        val res = this.mService?.appWebsocketAuth(args.appId)

        // Inject launcher env into web view
        this.injectHolochainClientEnv(args.appId, res!!.port, res!!.token)      
        
        val obj = JSObject() 
        obj.put("appWebsocketAuth", res!!.toJSObject())
        invoke.resolve(obj)       
    }

    private fun injectHolochainClientEnv(appId: String, appWebsocketPort: Int, appWebsocketToken: UByteArray) {
        // Declare js helper function for injecting holochain client env, bundled with dependencies
        this.webView.evaluateJavascript(this.injectHolochainClientEnvJavascript, null)

        // Inject holochain client env
        val tokenJsArray = appWebsocketToken.toMutableList().toJSArray() 
        this.webView.evaluateJavascript(
            """injectHolochainClientEnv("${appId}", ${appWebsocketPort}, ${tokenJsArray}) """, 
            null
        )
    }

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

    /// Start service, which then starts the holochain conductor on initialization
    private fun launchInternal() {
        // Create notification channel
        val notificationManager = activity.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
        notificationManager.createNotificationChannel(NotificationChannel(
            "HolochainServiceChannel",
            "Holochain Service",
            NotificationManager.IMPORTANCE_HIGH
        ))

        // Start service
        val intent = Intent(activity, HolochainService::class.java)
        activity.startForegroundService(intent)
        activity.bindService(intent, this.mConnection, 0)
    }
}
