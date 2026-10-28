# Keep UniFFI / JNA symbols used via reflection.
-keep class com.sun.jna.** { *; }
-keep class * implements com.sun.jna.** { *; }
