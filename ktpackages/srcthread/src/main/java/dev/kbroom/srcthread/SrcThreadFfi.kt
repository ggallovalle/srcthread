package dev.kbroom.srcthread

import android.util.Log
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import dev.kbroom.srcthread.uniffi.HostCallbacks
import java.io.Closeable
import dev.kbroom.srcthread.uniffi.SrcthreadFfi as NativeSrcthreadFfi

/**
 * [SrcThread] backed by a per-instance UniFFI handle. Registers host callbacks
 * so Rust mutations refresh [threads] / [greeting] as Compose [State].
 *
 * Call [close] (or let [rememberSrcThread] dispose) to destroy the native
 * handle when the instance is no longer needed.
 */
class SrcThreadFfi : SrcThread, Closeable {
    private val native = NativeSrcthreadFfi()
    private val greetingState: MutableState<String> = mutableStateOf(DefaultGreeting)
    private val threadsState: MutableState<List<String>> = mutableStateOf(emptyList())

    override val greeting: State<String> = greetingState
    override val threads: State<List<String>> = threadsState

    init {
        runCatching {
            native.setHostCallbacks(
                object : HostCallbacks {
                    override fun onThreadsChanged(threads: List<String>) {
                        threadsState.value = threads
                    }

                    override fun onLog(message: String) {
                        Log.i(TAG, message)
                    }
                },
            )
            greetingState.value = native.hello()
            threadsState.value = native.listThreads()
        }.onFailure { err ->
            Log.w(TAG, "native engine unavailable", err)
        }
    }

    override fun createThread(id: String) {
        runCatching { native.createThread(id) }
            .onFailure { err -> Log.w(TAG, "createThread failed", err) }
    }

    override fun close() {
        native.close()
    }

    companion object {
        private const val TAG = "srcthread"
        private const val DefaultGreeting = "srcthread"
    }
}
