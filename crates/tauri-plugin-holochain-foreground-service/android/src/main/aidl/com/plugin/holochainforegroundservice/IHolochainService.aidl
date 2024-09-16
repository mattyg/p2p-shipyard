package com.plugin.holochainforegroundservice;

parcelable InstallAppRequest;

interface IHolochainService {
    void shutdown();
    int getAdminPort();
    void installApp(in InstallAppRequest request);
}
