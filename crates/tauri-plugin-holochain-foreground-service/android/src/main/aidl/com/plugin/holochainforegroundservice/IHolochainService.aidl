package com.plugin.holochainforegroundservice;

parcelable InstallAppRequest;
parcelable ListInstalledAppsResponse;
parcelable AppInfoFfiExt;
parcelable AppWebsocketAuthFfiExt;

interface IHolochainService {
    void shutdown();
    int getAdminPort();
    void installApp(in InstallAppRequest request);
    List<AppInfoFfiExt> listInstalledApps();
    AppWebsocketAuthFfiExt appWebsocketAuth(String appId);
}
