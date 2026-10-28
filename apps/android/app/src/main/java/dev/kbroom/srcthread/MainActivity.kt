package dev.kbroom.srcthread

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import dev.kbroom.srcthread.uniffi.HostCallbacks
import dev.kbroom.srcthread.uniffi.createThread
import dev.kbroom.srcthread.uniffi.hello
import dev.kbroom.srcthread.uniffi.listThreads
import dev.kbroom.srcthread.uniffi.setHostCallbacks

/**
 * Compose hello Activity wired both ways to UniFFI:
 * Kotlin → Rust (`hello` / `listThreads` / `createThread`) and
 * Rust → Kotlin (`HostCallbacks`).
 */
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val ffiReady = registerHostCallbacks()
        setContent {
            MaterialTheme {
                Surface(modifier = Modifier.fillMaxSize()) {
                    HelloScreen(ffiReady = ffiReady)
                }
            }
        }
    }

    private fun registerHostCallbacks(): Boolean =
        runCatching {
            setHostCallbacks(
                object : HostCallbacks {
                    override fun onThreadsChanged(threads: List<String>) {
                        // Compose refreshes from the button handler after mutations.
                    }

                    override fun onLog(message: String) {
                        Log.i(TAG, message)
                    }
                },
            )
            true
        }.getOrDefault(false)

    companion object {
        private const val TAG = "srcthread"
    }
}

@Composable
private fun HelloScreen(ffiReady: Boolean) {
    var greeting by remember {
        mutableStateOf(if (ffiReady) queryHello() else OfflineGreeting)
    }
    var threads by remember {
        mutableStateOf(if (ffiReady) queryThreads() else emptyList())
    }
    var status by remember { mutableStateOf("query: hello + listThreads") }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(48.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp, Alignment.CenterVertically),
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        Text(text = greeting, style = MaterialTheme.typography.headlineMedium)
        Text(
            text = "threads: ${threads.joinToString()}",
            style = MaterialTheme.typography.bodyLarge,
        )
        Button(
            onClick = {
                val outcome = mutateCreateThread(ffiReady, threads.size)
                greeting = outcome.greeting
                threads = outcome.threads
                status = outcome.status
            },
        ) {
            Text("create thread")
        }
        Text(text = status, style = MaterialTheme.typography.bodyMedium)
    }
}

private data class MutationOutcome(
    val greeting: String,
    val threads: List<String>,
    val status: String,
)

private fun queryHello(): String =
    runCatching { hello() }.getOrElse { OfflineGreeting }

private fun queryThreads(): List<String> =
    runCatching { listThreads() }.getOrDefault(emptyList())

private fun mutateCreateThread(ffiReady: Boolean, threadCount: Int): MutationOutcome {
    if (!ffiReady) {
        return MutationOutcome(OfflineGreeting, emptyList(), "ffi offline — mise android-uniffi")
    }
    val id = "android-${threadCount + 1}"
    return runCatching { createThread(id) }
        .map {
            MutationOutcome(
                greeting = hello(),
                threads = listThreads(),
                status = "mutation: createThread $id",
            )
        }
        .getOrElse { err ->
            MutationOutcome(queryHello(), queryThreads(), "error: ${err.message}")
        }
}

private const val OfflineGreeting = "srcthread hello (ffi offline)"
