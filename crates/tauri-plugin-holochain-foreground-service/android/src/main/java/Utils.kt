package com.plugin.holochainforegroundservice

import app.tauri.plugin.JSObject
import app.tauri.plugin.JSArray
import kotlin.reflect.full.memberProperties
import kotlin.reflect.KProperty1
import android.util.Log

object Utils {
    inline fun <reified T : Any> toJSObject(data: T): JSObject {
        val obj = JSObject()
        val properties = data::class.memberProperties
        for (property in properties) { 
            val prop = property as? KProperty1<T, *>
            val value = prop?.get(data)
            when (value) {
                is String, is Int, is Long, is Double, is Boolean -> obj.put(property.name, value)
                is Enum<*> -> obj.put(property.name, value.name)
                null -> obj.put(property.name, null)
                is Collection<*> -> {
                    val jsValue = try {
                        (value as? Collection<Any>)?.toJSArray()
                    } catch (e: Exception) {
                        Log.e("toJSArray", "Error converting property ${property.name} to toJSArray", e)
                        null
                    }
                    obj.put(property.name, jsValue)
                }
                else -> {
                    val jsValue = try {
                        (value as? Any)?.toJSObject()
                    } catch (e: Exception) {
                        Log.e("toJSObject", "Error converting property ${property.name} to JSObject", e)
                        null
                    }
                    obj.put(property.name, jsValue)
                }
            }
        }
        return obj
    }

    inline fun <reified T : Collection<Any>> toJSArray(data: T): JSArray {
        val arr = JSArray()
        for (element in data) {
            when (element) {
                is String, is Int, is Long, is Double, is Float, is Boolean -> arr.put(element)
                is Enum<*> -> arr.put(element.name)
                is Collection<*> -> {
                    val jsValue = try {
                        (element as? Collection<Any>)?.toJSArray()
                    } catch (e: Exception) {
                        Log.e("toJSArray", "Error converting element ${element} to toJSArray", e)
                        null
                    }
                    arr.put(jsValue)
                }
                else -> {
                    val jsValue = try {
                        (element as? Any)?.toJSObject()
                    } catch (e: Exception) {
                        Log.e("toJSObject", "Error converting element ${element} to toJSObject", e)
                        null
                    }
                    arr.put(jsValue)
                }
            }
        }
        return arr
    }
}

fun Any.toJSObject(): JSObject = Utils.toJSObject(this)
fun Collection<Any>.toJSArray(): JSArray = Utils.toJSArray(this)
