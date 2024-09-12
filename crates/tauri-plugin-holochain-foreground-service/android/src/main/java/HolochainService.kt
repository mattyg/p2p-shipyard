package com.plugin.holochainforegroundservice

import android.util.Log
import android.app.Notification
import android.app.Service
import android.content.pm.ServiceInfo
import android.app.ForegroundServiceStartNotAllowedException
import android.os.Build
import android.os.IBinder
import androidx.core.app.NotificationCompat
import androidx.core.app.ServiceCompat
import android.content.Intent
import android.content.Context
import uniffi.holochain_manager_uniffi.HolochainRuntimeFfi
import uniffi.holochain_manager_uniffi.HolochainRuntimeFfiConfig
import uniffi.holochain_manager_uniffi.HolochainRuntimeFfiConfigException
import uniffi.holochain_manager_uniffi.HolochainRuntimeFfiException
import uniffi.holochain_manager_uniffi.AppInfoFfi
import kotlinx.coroutines.runBlocking

val NOTIFICATION_CHANNEl_ID = 98234982398

class HolochainService : Service() {
    private val LOG_TAG = "HolochainService"
    private val isAboveOrEqualAndroid10 = Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q
    public var runtime: HolochainRuntimeFfi? = null
    public var adminPort: UShort? = null

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        startForeground()
        return START_REDELIVER_INTENT
    }

    override fun onDestroy() {
        super.onDestroy()
    }

    override fun onBind(p0: Intent): IBinder? {
        return null
    }

    private fun startForeground() {
        try {
            // Create the notification to display while the service is running
            val notification = NotificationCompat.Builder(this, "HolochainServiceChannel")
                .setContentTitle("Holochain Conductor is Running")
                .build()
            startForeground(NOTIFICATION_CHANNEl_ID, notification)

            // Start holochain conductor
            val password = byteArrayOf(0x48, 101, 108, 108, 111)
            val config = HolochainRuntimeFfiConfig(
                "https://bootstrap.holo.host",
                "wss://signal.holo.host",
                getFilesDir().toString(),
            )
            this.runtime = runBlocking {
                var r: HolochainRuntimeFfi = HolochainRuntimeFfi.launch(password, config)
                r
            }
            Log.d(LOG_TAG, "Holochain started successfully")

            // Get admin port
            this.adminPort = runBlocking {
                var port: UShort? = runtime?.getAdminPort()
                port             
            }
            Log.d(LOG_TAG, "Admin Port: " + adminPort)
        } catch (e: Exception) {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S
                    && e is ForegroundServiceStartNotAllowedException) {
                // App not in a valid state to start foreground service
                // (e.g. started from bg)
            }
            // ...
        }
    }

    //public fun listInstalledApps() {
    //    val apps = runBlocking {
    //        this.runtime?.listInstalledApps()
    //    }
//
    //    return apps
    //}
}
