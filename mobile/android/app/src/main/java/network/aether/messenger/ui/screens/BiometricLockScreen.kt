package network.aether.messenger.ui.screens

import androidx.compose.animation.core.*
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Fingerprint
import androidx.compose.material.icons.filled.Security
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.scale
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import network.aether.messenger.ui.theme.*

@Composable
fun BiometricLockScreen(onUnlocked: () -> Unit) {
    val infiniteTransition = rememberInfiniteTransition(label = "pulse")
    val pulseScale by infiniteTransition.animateFloat(
        initialValue = 0.95f,
        targetValue = 1.05f,
        animationSpec = infiniteRepeatable(
            animation = tween(1200, easing = FastOutSlowInEasing),
            repeatMode = RepeatMode.Reverse
        ),
        label = "scale"
    )

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(DeepObsidian)
            .padding(24.dp),
        contentAlignment = Alignment.Center
    ) {
        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.SpaceBetween,
            modifier = Modifier.fillMaxHeight()
        ) {
            Spacer(modifier = Modifier.height(32.dp))

            // Logo & Title
            Column(horizontalAlignment = Alignment.CenterHorizontally) {
                Box(
                    modifier = Modifier
                        .size(88.dp)
                        .background(CardSurface, RoundedCornerShape(24.dp))
                        .border(1.5.dp, CyberEmerald.copy(alpha = 0.4f), RoundedCornerShape(24.dp)),
                    contentAlignment = Alignment.Center
                ) {
                    Icon(
                        imageVector = Icons.Default.Security,
                        contentDescription = "Aether Logo",
                        tint = CyberEmerald,
                        modifier = Modifier.size(44.dp)
                    )
                }

                Spacer(modifier = Modifier.height(16.dp))

                Text(
                    text = "أثيـر | AETHER",
                    fontSize = 24.sp,
                    color = Color.White,
                    style = MaterialTheme.typography.headlineMedium
                )
                Text(
                    text = "تشفير هجين • صفر خوادم • صفر هوية",
                    fontSize = 12.sp,
                    color = MutedSlate,
                    modifier = Modifier.padding(top = 4.dp)
                )
            }

            // Fingerprint Button
            Column(horizontalAlignment = Alignment.CenterHorizontally) {
                IconButton(
                    onClick = onUnlocked,
                    modifier = Modifier
                        .size(92.dp)
                        .scale(pulseScale)
                        .background(CardSurface, CircleShape)
                        .border(2.dp, CyberEmerald, CircleShape)
                ) {
                    Icon(
                        imageVector = Icons.Default.Fingerprint,
                        contentDescription = "Unlock",
                        tint = CyberEmerald,
                        modifier = Modifier.size(52.dp)
                    )
                }

                Spacer(modifier = Modifier.height(16.dp))

                Text(
                    text = "المس مستشعر البصمة لفك تشفير الجلسة",
                    fontSize = 13.sp,
                    color = MutedSlate
                )
            }

            // Footer
            Text(
                text = "StrongBox KeyStore • RAM Zeroize Active",
                fontSize = 11.sp,
                color = MutedSlate.copy(alpha = 0.6f),
                modifier = Modifier.padding(bottom = 16.dp)
            )
        }
    }
}
