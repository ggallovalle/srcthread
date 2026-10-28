package dev.kbroom.srcthread

import android.util.Log
import androidx.compose.runtime.MutableState
import androidx.compose.runtime.State
import androidx.compose.runtime.mutableStateOf
import dev.kbroom.srcthread.uniffi.HostCallbacks
import dev.kbroom.srcthread.uniffi.hello
import dev.kbroom.srcthread.uniffi.listThreads
import dev.kbroom.srcthread.uniffi.setHostCallbacks
import dev.kbroom.srcthread.uniffi.createThread as uniffiCreateThread

/**
 * [SrcThread] backed by UniFFI. Registers host callbacks so Rust mutations
 * refresh [threads] / [greeting] as Compose [State].
 */
class SrcThreadFfi : SrcThread {
    private val greetingState: MutableState<String> = mutableStateOf(DefaultGreeting)
    private val threadsState: MutableState<List<String>> = mutableStateOf(emptyList())

    override val greeting: State<String> = greetingState
    override val threads: State<List<String>> = threadsState

    init {
        runCatching {
            setHostCallbacks(
                object : HostCallbacks {
                    override fun onThreadsChanged(threads: List<String>) {
                        threadsState.value = threads
                    }

                    override fun onLog(message: String) {
                        Log.i(TAG, message)
                    }
                },
            )
            greetingState.value = hello()
            threadsState.value = listThreads()
        }.onFailure { err ->
            Log.w(TAG, "native engine unavailable", err)
        }
    }

    override fun createThread(id: String) {
        runCatching { uniffiCreateThread(id) }
            .onFailure { err -> Log.w(TAG, "createThread failed", err) }
    }

    companion object {
        private const val TAG = "srcthread"
        private const val DefaultGreeting = "srcthread"
    }
}
