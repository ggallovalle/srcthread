package dev.kbroom.srcthread

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.border
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxHeight
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
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

/** Compose hello Activity with a dynamic row of independent [SrcThread] engines. */
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MaterialTheme {
                Surface(modifier = Modifier.fillMaxSize()) {
                    EngineInstanceRow()
                }
            }
        }
    }
}

private val InitialInstanceIds = listOf(0, 1, 2)

@Composable
private fun EngineInstanceRow() {
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
        Row(
            modifier = Modifier
                .weight(1f)
                .horizontalScroll(rememberScrollState()),
            horizontalArrangement = Arrangement.spacedBy(12.dp),
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
            .width(220.dp)
            .fillMaxHeight()
            .border(1.dp, MaterialTheme.colorScheme.outline)
            .padding(12.dp),
        verticalArrangement = Arrangement.spacedBy(8.dp, Alignment.Top),
        horizontalAlignment = Alignment.CenterHorizontally,
    ) {
        Text(text = label, style = MaterialTheme.typography.titleMedium)
        Text(text = greeting, style = MaterialTheme.typography.bodyLarge)
        Text(
            text = "threads: ${threads.joinToString()}",
            style = MaterialTheme.typography.bodySmall,
        )
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
