# ProGuard Rules for Aether Messenger

# Keep JNA native reflection classes
-keep class com.sun.jna.** { *; }
-keepclassmembers class * extends com.sun.jna.** { public *; }

# Keep UniFFI generated classes
-keep class uniffi.** { *; }
-keep class network.aether.** { *; }
