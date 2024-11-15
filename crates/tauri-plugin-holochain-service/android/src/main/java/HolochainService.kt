package com.plugin.holochain_service

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
import uniffi.holochain_runtime_uniffi.HolochainRuntimeFfi
import uniffi.holochain_runtime_uniffi.HolochainRuntimeFfiConfig
import uniffi.holochain_runtime_uniffi.HolochainRuntimeFfiConfigException
import uniffi.holochain_runtime_uniffi.HolochainRuntimeFfiException
import uniffi.holochain_runtime_uniffi.AppInfoFfi
import uniffi.holochain_runtime_uniffi.CellIdFfi
import uniffi.holochain_runtime_uniffi.ZomeCallUnsignedFfi
import uniffi.holochain_runtime_uniffi.GossipArcClampFfi
import kotlinx.coroutines.runBlocking
import java.io.IOException
import android.os.SharedMemory
import java.nio.ByteBuffer

val NOTIFICATION_CHANNEl_ID: Int = 9823498

class HolochainService : Service() {
    /// The uniffi-generated holochain runtime bindings
    public var runtime: HolochainRuntimeFfi? = null

    /// Holochain conductor admin websocket port
    public var runtimeAdminWebsocketPort: UShort? = null

    private val LOG_TAG = "HolochainService"
    
    /// The IPC receiver that other activities can call into
    private val binder = object : IHolochainService.Stub() {
        
        /// Get Current Admin Port
        override fun getAdminPort(): Int {
            Log.d("IHolochainService", "getAdminPort")
            if(runtimeAdminWebsocketPort is UShort) {
                return runtimeAdminWebsocketPort!!.toInt()
            } else {
                return -1
            }
        }

        /// Stop the service
        override fun shutdown() {
            Log.d("IHolochainService", "shutdown")
            var x = stopForeground()
        }
        
        /// Install an app
        override fun installApp(
            request: InstallAppRequestAidl
        ) {
            Log.d("IHolochainService", "installApp")
            // Read appBundle bytes from shared memory
            val appBundleBuffer: ByteBuffer = request.appBundleSharedMemory.mapReadOnly()
            val appBundleBytes: ByteArray = appBundleBuffer.toByteArray()
            
            // Call install app
            runBlocking {
                runtime?.installApp(request.appId, appBundleBytes, request.membraneProofs, request.agent, request.networkSeed)
            }
        }

        /// Uninstall an app
        override fun uninstallApp(
            appId: String
        ) {
            Log.d("IHolochainService", "uninstallApp")
            runBlocking {
                runtime?.uninstallApp(appId)
            }
        }

        /// Enable an installed app
        override fun enableApp(
            appId: String
        ) {
            Log.d("IHolochainService", "enableApp")
            runBlocking {
                runtime?.enableApp(appId)
            }
        }

        /// Disable an installed app
        override fun disableApp(
            appId: String
        ) {
            Log.d("IHolochainService", "disableApp")
            runBlocking {
                runtime?.disableApp(appId)
            }
        }

        /// List installed apps
        override fun listInstalledApps(): List<AppInfoFfiAidl> {
            Log.d("IHolochainService", "listInstalledApps")
            return runBlocking {
                runtime?.listInstalledApps()?.map { 
                    AppInfoFfiAidl(
                        it.installedAppId, 
                        it.cellInfo,
                        AppInfoStatusFfiAidl(it.status::class.simpleName!!),
                        it.agentPubKey
                    ) 
                } ?: emptyList<AppInfoFfiAidl>()
            }
        }

        /// Is app installed
        override fun isAppInstalled(appId: String): Boolean {
            Log.d("IHolochainService", "isAppInstalled")
            return runBlocking {
                Log.d(LOG_TAG, "isAppInstalled 2")
                val res = runtime?.isAppInstalled(appId)!!
                Log.d(LOG_TAG, "isAppInstalled 3 " + res)
                res
            }
        }

        /// Get or create an app websocket with an authenticated token
        override fun appWebsocketAuth(appId: String): AppWebsocketAuthFfiAidl {
            Log.d("IHolochainService", "appWebsocketAuth")
            return runBlocking {
                val res = runtime?.appWebsocketAuth(appId)!!
                AppWebsocketAuthFfiAidl(res.appId, res.port.toInt(), res.token.toUByteArray())
            }
        }

        /// Sign a zome call
        override fun signZomeCall(request: SignZomeCallRequestAidl): ZomeCallSignedFfiAidl {
            Log.d("IHolochainService", "signZomeCall")
            return runBlocking {
                val res = runtime?.signZomeCall(ZomeCallUnsignedFfi(
                    request.provenance,
                    CellIdFfi(
                        request.cellIdDnaHash,
                        request.cellIdAgentPubKey,
                    ),
                    request.zomeName,
                    request.fnName,
                    request.capSecret,
                    request.payload,
                    request.nonce,
                    request.expiresAt,
                ))!!
                
                ZomeCallSignedFfiAidl(
                    res.cellId.dnaHash,
                    res.cellId.agentPubKey,
                    res.zomeName,
                    res.fnName,
                    res.payload,
                    res.capSecret,
                    res.provenance,
                    res.signature,
                    res.nonce,
                    res.expiresAt,
                )
            }
        }
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        startForeground()
        return START_REDELIVER_INTENT
    }

    override fun onDestroy() {
        super.onDestroy()
    }

    override fun onBind(intent: Intent?): IBinder? = binder

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
                "https://bootstrap-0.infra.holochain.org",
                "wss://sbd.holo.host",
                getFilesDir().toString(),
                listOf<String>("stun:stun-0.main.infra.holo.host:443", "stun:stun-1.main.infra.holo.host:443"),
                GossipArcClampFfi.FULL,
                true
            );
            this.runtime = runBlocking {
                var r: HolochainRuntimeFfi = HolochainRuntimeFfi.launch(password, config)
                r
            }
            Log.d(LOG_TAG, "Holochain started successfully")

            // Get admin port
            this.runtimeAdminWebsocketPort = runBlocking {
                var port: UShort? = runtime?.getAdminPort()
                port     
            }
            Log.d(LOG_TAG, "Holochain admin port " + this.runtimeAdminWebsocketPort)
        } catch (e: Exception) {
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S
                    && e is ForegroundServiceStartNotAllowedException) {
                // App not in a valid state to start foreground service
                // (e.g. started from bg)
            }
            // ...
        }
        Log.d(LOG_TAG, "Admin Port: " + this.runtimeAdminWebsocketPort)
    }

    fun stopForeground() {
        // Shutdown conductor
        runBlocking {
            runtime?.shutdown()
        }
        
        this.runtime = null
        this.runtimeAdminWebsocketPort = null
        
        // Stop service
        super.stopForeground(true)
        stopSelf()
    }
}
