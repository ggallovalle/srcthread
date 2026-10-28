package dev.kbroom.srcthread

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.ExperimentalLayoutApi
import androidx.compose.foundation.layout.FlowRow
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.heightIn
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.SuggestionChip
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.key
import androidx.compose.runtime.mutableIntStateOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

/** Compose hello Activity with a dynamic list of independent [SrcThread] engines. */
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MaterialTheme {
                Surface(modifier = Modifier.fillMaxSize()) {
                    EngineInstanceList()
                }
            }
        }
    }
}

private val InitialInstanceIds = listOf(0, 1, 2)

@Composable
private fun EngineInstanceList() {
    var instanceIds by remember { mutableStateOf(InitialInstanceIds) }
    var nextId by remember { mutableIntStateOf(InitialInstanceIds.size) }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        Text(
            text = "srcthread ${BuildConfig.VERSION_NAME}",
            style = MaterialTheme.typography.labelMedium,
        )
        Button(
            onClick = {
                instanceIds = instanceIds + nextId
                nextId += 1
            },
        ) {
            Text("Add instance")
        }
        Column(
            modifier = Modifier
                .weight(1f)
                .fillMaxWidth()
                .verticalScroll(rememberScrollState()),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            instanceIds.forEach { id ->
                key(id) {
                    EnginePanel(
                        label = "Engine $id",
                        onRemove = { instanceIds = instanceIds - id },
                    )
                }
            }
        }
    }
}

@OptIn(ExperimentalLayoutApi::class)
@Composable
private fun EnginePanel(
    label: String,
    onRemove: () -> Unit,
    engine: SrcThread = rememberSrcThread(),
) {
    val greeting by engine.greeting
    val threads by engine.threads

    Column(
        modifier = Modifier
            .fillMaxWidth()
            .border(1.dp, MaterialTheme.colorScheme.outline)
            .padding(12.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp, Alignment.Top),
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        Text(text = label, style = MaterialTheme.typography.titleMedium)
        Text(text = greeting, style = MaterialTheme.typography.bodyLarge)
        Text(
            text = "Threads (${threads.size})",
            modifier = Modifier.fillMaxWidth(),
            style = MaterialTheme.typography.labelLarge,
        )
        FlowRow(
            modifier = Modifier
                .fillMaxWidth()
                .heightIn(max = ThreadsAreaMaxHeight)
                .verticalScroll(rememberScrollState()),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
            verticalArrangement = Arrangement.spacedBy(4.dp),
        ) {
            threads.forEach { threadId ->
                SuggestionChip(
                    onClick = {},
                    label = { Text(threadId) },
                )
            }
        }
        Button(
            onClick = {
                engine.createThread("$label-${threads.size + 1}")
            },
        ) {
            Text("create thread")
        }
        Button(onClick = onRemove) {
            Text("Remove")
        }
    }
}

private val ThreadsAreaMaxHeight = 160.dp
