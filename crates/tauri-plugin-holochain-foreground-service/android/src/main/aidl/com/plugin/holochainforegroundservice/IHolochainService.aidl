package com.plugin.holochainforegroundservice;

interface IHolochainService {
    void shutdown();
    int getAdminPort();
    long installApp(request: InstallAppRequest);
}
