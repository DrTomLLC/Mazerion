#!/usr/bin/env kotlin

@file:DependsOn("net.java.dev.jna:jna:5.14.0")

// Copy the generated .kt file content here or load it
// For now, just verify the DLL loads:

import com.sun.jna.Native
import com.sun.jna.Library

interface MazerionLib : Library {
    companion object {
        val INSTANCE: MazerionLib = Native.load("mazerion_ffi", MazerionLib::class.java)
    }
}

fun main() {
    println("Testing Mazerion FFI...")
    try {
        // This will fail until we copy the DLL to the right place
        println("Library loaded successfully!")
    } catch (e: Exception) {
        println("Error: ${e.message}")
    }
}