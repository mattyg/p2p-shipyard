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
  val token: UByteArray,
): Parcelable

@Parcelize
data class SignZomeCallRequest(
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
data class ZomeCallSignedFfiExt(
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