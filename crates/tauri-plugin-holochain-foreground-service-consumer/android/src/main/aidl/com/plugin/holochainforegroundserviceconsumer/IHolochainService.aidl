package com.plugin.holochainforegroundservice;

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
    AppWebsocketAuthFfiAidl appWebsocketAuth(String appId);
    ZomeCallSignedFfiAidl signZomeCall(in SignZomeCallRequestAidl request);
}
