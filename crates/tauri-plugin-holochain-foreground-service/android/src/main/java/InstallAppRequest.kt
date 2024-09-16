package com.plugin.holochainforegroundservice

import kotlinx.parcelize.Parcelize
import android.os.Parcelable
import app.tauri.annotation.InvokeArg

@InvokeArg
@Parcelize
data class InstallAppRequest(
  val appId: String,
  val appBundleBytes: ByteArray,
  val membraneProofs: Map<String, ByteArray>,
  val agent: ByteArray?,
  val networkSeed: String?,
): Parcelable