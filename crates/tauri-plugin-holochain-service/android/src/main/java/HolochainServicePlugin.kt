package com.plugin.holochain_service

import android.app.Activity
import android.webkit.WebView
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@TauriPlugin
class HolochainServicePlugin(private val activity: Activity): Plugin(activity) {
    private lateinit var webView: WebView
    private lateinit var injectHolochainClientEnvJavascript: String
    private lateinit var holochainServiceConsumer: HolochainServiceConsumer

    /**
     * Load the plugin, start the service
     */
    override fun load(webView: WebView) {
        super.load(webView)
        this.webView = webView

        // Load holochain client injected javascript from resource file
        val resourceInputStream = this.activity.resources.openRawResource(R.raw.injectholochainclientenv)
        this.injectHolochainClientEnvJavascript = resourceInputStream.bufferedReader().use { it.readText() }

        // Start the service
        this.holochainServiceConsumer = HolochainServiceConsumer(this.activity)
        this.holochainServiceConsumer.launch()
    }

    /**
     * Start the conductor service
     * - Starts the foreground service
     * - Launches a conductor
     * - Creates an admin websocket
     */
    @Command
    fun launch(invoke: Invoke) {
        val args = invoke.parseArgs(HolochainArgs::class.java)
        this.holochainServiceConsumer.launch()
        invoke.resolve()
    }
    
    /**
     * Shutdown the conductor service
     */
    @Command
    fun shutdown(invoke: Invoke) {
        this.holochainServiceConsumer.shutdown()
        invoke.resolve()
    }

    /**
     *  Get the conductor admin websocket port
     */
    @Command
    fun getAdminPort(invoke: Invoke) {
        val res = this.holochainServiceConsumer.getAdminPort()
        val obj = JSObject()
        obj.put("port", res)
        invoke.resolve(obj)
    }

    /**
     * Install an app into the conductor
     */
    @Command
    fun installApp(invoke: Invoke) {
        val args = invoke.parseArgs(InstallAppRequestArgs::class.java)
        this.holochainServiceConsumer.installApp(args)
        invoke.resolve()
    }

    /**
     * Is an app with the given app_id installed
     */
    @Command
    fun isAppInstalled(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdRequestArgs::class.java)
        val res = this.holochainServiceConsumer.isAppInstalled(args.appId)

        val obj = JSObject()
        obj.put("installed", res)
        invoke.resolve(obj)
    }

    /**
     * Uninstall an installed app
     */
    @Command
    fun uninstallApp(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdRequestArgs::class.java)
        this.holochainServiceConsumer.uninstallApp(args.appId)
        invoke.resolve()
    }

    /**
     * Enable an installed app
     */
    @Command
    fun enableApp(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdRequestArgs::class.java)
        this.holochainServiceConsumer.enableApp(args.appId)
        invoke.resolve()
    }

    /**
     * Disable an installed app
     */
    @Command
    fun disableApp(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdRequestArgs::class.java)
        this.holochainServiceConsumer.disableApp(args.appId)
        invoke.resolve()
    }

    /**
     * List installed apps in conductor
     */
    @Command
    fun listInstalledApps(invoke: Invoke) {
        val res = this.holochainServiceConsumer.listInstalledApps()
        val obj = JSObject() 
        obj.put("installedApps", res.toJSArray())
        invoke.resolve(obj)
    }

    /**
     * Get or create an app websocket with authentication token
     */
    @OptIn(ExperimentalUnsignedTypes::class)
    @Command
    fun appWebsocketAuth(invoke: Invoke) {
        val args = invoke.parseArgs(AppIdRequestArgs::class.java)
        val res = this.holochainServiceConsumer.appWebsocketAuth(args.appId)

        // Inject launcher env into web view
        this.injectHolochainClientEnv(args.appId, res.port, res.token)
        
        val obj = JSObject() 
        obj.put("appWebsocketAuth", res.toJSObject())
        invoke.resolve(obj)       
    }

    /**
     * Sign Zome Call
     */
    @Command
    fun signZomeCall(invoke: Invoke) {
        val args = invoke.parseArgs(SignZomeCallRequestArgs::class.java)
        val res = this.holochainServiceConsumer.signZomeCall(
            SignZomeCallRequestAidl(
                args.provenance,
                args.cellIdDnaHash,
                args.cellIdAgentPubKey,
                args.zomeName,
                args.fnName,
                args.capSecret,
                args.payload,
                args.nonce,
                args.expiresAt
            )
        )
        invoke.resolve(res.toJSObject())
    }

    /**
     * Inject magic holochain-client-js variables into webview window
     */
    @OptIn(ExperimentalUnsignedTypes::class)
    private fun injectHolochainClientEnv(appId: String, appWebsocketPort: Int, appWebsocketToken: UByteArray) {
        // Declare js helper function for injecting holochain client env, bundled with dependencies
        this.webView.evaluateJavascript(this.injectHolochainClientEnvJavascript, null)

        // Inject holochain client env
        val tokenJsArray = appWebsocketToken.toMutableList().toJSArray() 
        this.webView.evaluateJavascript(
            """injectHolochainClientEnv("$appId", ${appWebsocketPort}, ${tokenJsArray}) """,
            null
        )
    }
}
