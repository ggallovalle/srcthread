package dev.kbroom.srcthread

import androidx.compose.runtime.Composable
import androidx.compose.runtime.remember

/** Remember a [SrcThread] backed by the native UniFFI engine for this composition. */
@Composable
fun rememberSrcThread(): SrcThread = remember { SrcThreadFfi() }
