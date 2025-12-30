# Keep Mazerion FFI classes
-keep class mazerion.** { *; }
-keep class uniffi.** { *; }

# Keep JNA
-keep class com.sun.jna.** { *; }
-keep class * implements com.sun.jna.** { *; }
-dontwarn com.sun.jna.**

# Keep Kotlin metadata
-keepattributes *Annotation*
-keepattributes Signature
-keepattributes InnerClasses
-keepattributes EnclosingMethod

# Keep data classes
-keepclassmembers class * {
    @kotlinx.serialization.SerialName <fields>;
}