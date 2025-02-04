package com.plugin.holochain_service;

parcelable InstallAppRequestAidl;
parcelable ListInstalledAppsResponse;
parcelable AppInfoFfiAidl;
parcelable AppWebsocketAuthFfiAidl;
parcelable SignZomeCallRequestAidl;
parcelable ZomeCallSignedFfiAidl;

interface IHolochainService {
    void shutdown();
    int getAdminPort();
    void installApp(in InstallAppRequestAidl request);
    void uninstallApp(String appId);
    void enableApp(String appId);
    void disableApp(String appId);
    List<AppInfoFfiAidl> listInstalledApps();
    boolean isAppInstalled(String appId);
    AppWebsocketAuthFfiAidl appWebsocketAuth(String appId);
    ZomeCallSignedFfiAidl signZomeCall(in SignZomeCallRequestAidl request);
}
