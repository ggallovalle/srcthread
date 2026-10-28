package dev.kbroom.srcthread

import android.os.Bundle
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
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

/** Compose hello Activity driven by [SrcThread] state. */
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MaterialTheme {
                Surface(modifier = Modifier.fillMaxSize()) {
                    HelloScreen()
                }
            }
        }
    }
}

@Composable
private fun HelloScreen(engine: SrcThread = rememberSrcThread()) {
    val greeting by engine.greeting
    val threads by engine.threads

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
                engine.createThread("android-${threads.size + 1}")
            },
        ) {
            Text("create thread")
        }
    }
}
