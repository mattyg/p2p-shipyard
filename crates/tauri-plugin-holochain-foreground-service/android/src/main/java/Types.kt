package com.plugin.holochainforegroundservice

import kotlinx.parcelize.Parcelize
import android.os.Parcelable

@Parcelize
data class InstallAppRequest(
  val appId: String,
  val appBundleBytes: ByteArray,
  val membraneProofs: Map<String, ByteArray>,
  val agent: ByteArray?,
  val networkSeed: String?,
): Parcelable

@Parcelize
data class AppInfoFfiExt(
  val installedAppId: String
): Parcelable

@Parcelize
data class AppWebsocketAuthFfiExt(
  val appId: String,
  val port: Int,
  val token: ByteArray,
): Parcelable