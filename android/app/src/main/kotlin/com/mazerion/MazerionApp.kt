package com.mazerion

import android.app.Application
import android.util.Log
import uniffi.mazerion_ffi.version

class MazerionApp : Application() {
    override fun onCreate() {
        super.onCreate()

        try {
            System.loadLibrary("mazerion_ffi")
            Log.i("Mazerion", "Native library loaded successfully")

            val ver = version()
            Log.i("Mazerion", "Mazerion version: $ver")

        } catch (e: Exception) {
            Log.e("Mazerion", "Failed to initialize: ${e.message}", e)
        }
    }
}