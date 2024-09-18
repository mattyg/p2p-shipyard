package com.plugin.holochainforegroundservice

import kotlinx.parcelize.Parcelize
import android.os.Parcelable

@Parcelize
data class InstallAppRequestAidl(
  val appId: String,
  val appBundleBytes: ByteArray,
  val membraneProofs: Map<String, ByteArray>,
  val agent: ByteArray?,
  val networkSeed: String?,
): Parcelable

@Parcelize
data class AppInfoFfiAidl(
  val installedAppId: String
): Parcelable

@Parcelize
data class AppWebsocketAuthFfiAidl(
  val appId: String,
  val port: Int,
  val token: UByteArray,
): Parcelable

@Parcelize
data class SignZomeCallRequestAidl(
  val provenance: ByteArray,
  val cellIdDnaHash: ByteArray,
  val cellIdAgentPubKey: ByteArray,
  val zomeName: String,
  val fnName: String,
  val capSecret: ByteArray?,
  val payload: ByteArray,
  val nonce: ByteArray,
  val expiresAt: Long,
): Parcelable

@Parcelize
data class ZomeCallSignedFfiAidl(
  val cellIdDnaHash: ByteArray,
  val cellIdAgentPubKey: ByteArray,
  val zomeName: String,
  val fnName: String,
  val payload: ByteArray,
  val capSecret: ByteArray?,
  val provenance: ByteArray,
  val signature: ByteArray,
  val nonce: ByteArray,
  val expiresAt: Long,
) : Parcelable