package dev.kbroom.srcthread

import androidx.compose.runtime.Composable
import androidx.compose.runtime.DisposableEffect
import androidx.compose.runtime.remember

/**
 * Remember a [SrcThread] backed by a native UniFFI engine for this composition.
 * Destroys the native handle when the composition leaves.
 */
@Composable
fun rememberSrcThread(): SrcThread {
    val engine = remember { SrcThreadFfi() }
    DisposableEffect(engine) {
        onDispose { engine.close() }
    }
    return engine
}
