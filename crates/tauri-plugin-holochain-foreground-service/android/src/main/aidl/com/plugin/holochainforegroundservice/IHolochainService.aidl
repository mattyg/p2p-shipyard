package com.plugin.holochainforegroundservice;

parcelable InstallAppRequest;
parcelable ListInstalledAppsResponse;
parcelable AppInfoFfiExt;

interface IHolochainService {
    void shutdown();
    int getAdminPort();
    void installApp(in InstallAppRequest request);
    List<AppInfoFfiExt> listInstalledApps();
}
