package network.aether.messenger

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.runtime.*
import network.aether.messenger.ui.screens.BiometricLockScreen
import network.aether.messenger.ui.screens.ChatScreen
import network.aether.messenger.ui.theme.AetherColorScheme
import androidx.compose.material3.MaterialTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            MaterialTheme(colorScheme = AetherColorScheme) {
                var isUnlocked by remember { mutableStateOf(false) }

                if (isUnlocked) {
                    ChatScreen(onLockClicked = { isUnlocked = false })
                } else {
                    BiometricLockScreen(onUnlocked = { isUnlocked = true })
                }
            }
        }
    }
}
