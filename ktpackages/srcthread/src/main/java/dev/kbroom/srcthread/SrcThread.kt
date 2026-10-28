package dev.kbroom.srcthread

import androidx.compose.runtime.State

/**
 * Compose-facing engine handle: product state plus mutations.
 *
 * UniFFI callbacks and native load details stay behind [SrcThreadFfi].
 */
interface SrcThread {
    val greeting: State<String>
    val threads: State<List<String>>

    fun createThread(id: String)
}
