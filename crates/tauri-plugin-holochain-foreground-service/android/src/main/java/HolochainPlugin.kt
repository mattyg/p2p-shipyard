package com.plugin.holochainforegroundservice

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
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import android.app.NotificationChannel
import android.app.NotificationManager
import android.util.Log
import android.webkit.WebView
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.delay

@InvokeArg
class HolochainArgs {
}

@TauriPlugin
class HolochainPlugin(private val activity: Activity): Plugin(activity) {
    private var mService: IHolochainService? = null

    private val LOG_TAG = "HolochainPlugin"
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

        // Start the service
        runBlocking {
            launchInternal()
        }
    }

    /// Start the service
    @Command
    fun launch(invoke: Invoke) {
        val args = invoke.parseArgs(HolochainArgs::class.java)
        launchInternal()
        invoke.resolve()
    }
    
    /// Stop the service
    @Command
    fun shutdown(invoke: Invoke) {
        val args = invoke.parseArgs(HolochainArgs::class.java)
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
        val args = invoke.parseArgs(InstallAppRequest::class.java)
        this.mService?.installApp(args)
        invoke.resolve()
    }

    /// Create a new app websocket
    @Command
    fun createAppWebsocket(invoke: Invoke) {
        val args = invoke.parseArgs(HolochainArgs::class.java)

        // Create app websocket
        /*
            TODO: return LauncherEnvironment
            export interface LauncherEnvironment {
                APP_INTERFACE_PORT?: number;
                ADMIN_INTERFACE_PORT?: number;
                INSTALLED_APP_ID?: InstalledAppId;
                APP_INTERFACE_TOKEN?: AppAuthenticationToken;
        } */        
        invoke.resolve()
    }

    @Command
    fun signZomeCall(invoke: Invoke) {
        val args = invoke.parseArgs(HolochainArgs::class.java)

        // Create app websocket
        /*
            TODO: return LauncherEnvironment
            export interface LauncherEnvironment {
                APP_INTERFACE_PORT?: number;
                ADMIN_INTERFACE_PORT?: number;
                INSTALLED_APP_ID?: InstalledAppId;
                APP_INTERFACE_TOKEN?: AppAuthenticationToken;
        } */        
        invoke.resolve()
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
