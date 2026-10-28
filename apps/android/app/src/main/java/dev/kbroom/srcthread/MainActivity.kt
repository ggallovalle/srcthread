package dev.kbroom.srcthread

import android.app.Activity
import android.os.Bundle
import android.widget.TextView

/**
 * Hello Activity. After `./gradlew bootstrapUniffi cargoNdkUniffi`, call
 * UniFFI `hello()` from `dev.kbroom.srcthread.uniffi` instead of this label.
 */
class MainActivity : Activity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val text = TextView(this).apply {
            text = "srcthread hello"
            textSize = 24f
            setPadding(48, 48, 48, 48)
        }
        setContentView(text)
    }
}
