package network.aether.messenger.ui.screens

import androidx.compose.animation.*
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.*
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import kotlinx.coroutines.delay
import network.aether.messenger.ui.theme.*

data class Message(
    val id: String,
    val text: String,
    val isOutgoing: Boolean,
    val time: String,
    val isMedia: Boolean = false
)

@Composable
fun ChatScreen(onLockClicked: () -> Unit) {
    var inputText by remember { mutableStateOf("") }
    var showAttachmentSheet by remember { mutableStateOf(false) }
    var dummyCount by remember { mutableLongStateOf(14892L) }

    val messages = remember {
        mutableStateListOf(
            Message("1", "تم إنشاء الجلسة بمفاتيح هجينة (Kyber + X25519) عبر شبكة Nym.", false, "21:28"),
            Message("2", "صورة مشفرة (1.2 MB) مقطعة إلى حزم Sphinx متساوية الحجم.", true, "21:29", isMedia = true)
        )
    }

    // محاكاة تدفق الحزم الوهمية
    LaunchedEffect(Unit) {
        while (true) {
            delay(2800)
            dummyCount += 1
        }
    }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(DeepObsidian)
    ) {
        // 1. Top Header Bar
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .background(CardSurface)
                .padding(horizontal = 16.dp, vertical = 10.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            IconButton(onClick = onLockClicked) {
                Icon(Icons.Default.ArrowBack, contentDescription = "Lock", tint = Color.White)
            }

            Box(
                modifier = Modifier
                    .size(40.dp)
                    .background(Color(0xFF1E293B), CircleShape),
                contentAlignment = Alignment.Center
            ) {
                Text("ع", color = CyberEmerald, fontSize = 16.sp)
            }

            Spacer(modifier = Modifier.width(12.dp))

            Column(modifier = Modifier.weight(1f)) {
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Text("علي (Node #702)", color = Color.White, fontSize = 14.sp)
                    Spacer(modifier = Modifier.width(6.dp))
                    Text(
                        "PQ-Hybrid",
                        color = CyberEmerald,
                        fontSize = 9.sp,
                        fontFamily = FontFamily.Monospace,
                        modifier = Modifier
                            .background(CyberEmerald.copy(alpha = 0.15f), RoundedCornerShape(4.dp))
                            .padding(horizontal = 4.dp, vertical = 2.dp)
                    )
                }
                Text(
                    "Cover Traffic: 1.2 pkt/s",
                    color = MutedSlate,
                    fontSize = 10.sp,
                    fontFamily = FontFamily.Monospace
                )
            }

            // Quick RAM Purge Button
            IconButton(
                onClick = {
                    messages.clear()
                    onLockClicked()
                },
                modifier = Modifier
                    .size(36.dp)
                    .background(Color.Red.copy(alpha = 0.15f), CircleShape)
            ) {
                Icon(Icons.Default.LocalFireDepartment, contentDescription = "Zeroize", tint = Color.Red)
            }
        }

        // 2. Mixnet Status Line
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .background(Color.Black.copy(alpha = 0.4f))
                .padding(horizontal = 16.dp, vertical = 4.dp),
            horizontalArrangement = Arrangement.SpaceBetween
        ) {
            Text("NYM SPHINX: حزمة وهمية #$dummyCount", color = CyberEmerald, fontSize = 10.sp, fontFamily = FontFamily.Monospace)
            Text("Ratchet #42 • PFS", color = MutedSlate, fontSize = 10.sp, fontFamily = FontFamily.Monospace)
        }

        // 3. Messages List
        LazyColumn(
            modifier = Modifier
                .weight(1f)
                .fillMaxWidth()
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp)
        ) {
            items(messages) { msg ->
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = if (msg.isOutgoing) Arrangement.End else Arrangement.Start
                ) {
                    Column(horizontalAlignment = if (msg.isOutgoing) Alignment.End else Alignment.Start) {
                        Surface(
                            color = if (msg.isOutgoing) OutgoingBubble else IncomingBubble,
                            shape = RoundedCornerShape(16.dp),
                            border = if (msg.isOutgoing) androidx.compose.foundation.BorderStroke(1.dp, CyberEmerald.copy(alpha = 0.3f)) else null,
                            modifier = Modifier.widthIn(max = 280.dp)
                        ) {
                            if (msg.isMedia) {
                                Column(modifier = Modifier.padding(10.dp), horizontalAlignment = Alignment.CenterHorizontally) {
                                    Icon(Icons.Default.Image, contentDescription = null, tint = CyberEmerald, modifier = Modifier.size(36.dp))
                                    Text(msg.text, color = Color.White, fontSize = 11.sp)
                                }
                            } else {
                                Text(
                                    text = msg.text,
                                    color = Color.White,
                                    fontSize = 13.sp,
                                    modifier = Modifier.padding(horizontal = 14.dp, vertical = 10.dp)
                                )
                            }
                        }

                        Text(
                            text = "${msg.time} ${if (msg.isOutgoing) "• تم التسليم ✓✓" else ""}",
                            color = if (msg.isOutgoing) CyberEmerald else MutedSlate,
                            fontSize = 9.sp,
                            fontFamily = FontFamily.Monospace,
                            modifier = Modifier.padding(top = 2.dp, start = 4.dp, end = 4.dp)
                        )
                    }
                }
            }
        }

        // 4. WhatsApp-Style Attachments Drawer
        AnimatedVisibility(visible = showAttachmentSheet) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(CardSurface)
                    .padding(vertical = 12.dp),
                horizontalArrangement = Arrangement.SpaceEvenly
            ) {
                AttachmentItem(Icons.Default.Photo, "صور وفيديو", CyberEmerald) {
                    messages.add(Message("${messages.size + 1}", "صورة مشفرة لمرة واحدة", true, "21:31", isMedia = true))
                    showAttachmentSheet = false
                }
                AttachmentItem(Icons.Default.Description, "مستند مشفر", QuantumCyan) {
                    messages.add(Message("${messages.size + 1}", "مستند PDF مشفر", true, "21:31", isMedia = true))
                    showAttachmentSheet = false
                }
                AttachmentItem(Icons.Default.CameraAlt, "كاميرا", Color(0xFF818CF8)) {
                    messages.add(Message("${messages.size + 1}", "لقطة كاميرا سريعة الزوال", true, "21:31", isMedia = true))
                    showAttachmentSheet = false
                }
            }
        }

        // 5. Input Bar
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .background(CardSurface)
                .padding(horizontal = 10.dp, vertical = 8.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            IconButton(
                onClick = { showAttachmentSheet = !showAttachmentSheet },
                modifier = Modifier
                    .size(38.dp)
                    .background(Color.Black.copy(alpha = 0.4f), CircleShape)
            ) {
                Icon(
                    Icons.Default.Add,
                    contentDescription = "Attachments",
                    tint = if (showAttachmentSheet) CyberEmerald else Color.White
                )
            }

            Spacer(modifier = Modifier.width(8.dp))

            TextField(
                value = inputText,
                onValueChange = { inputText = it },
                placeholder = { Text("رسالة مشفرة بنظام Sphinx...", color = MutedSlate, fontSize = 13.sp) },
                colors = TextFieldDefaults.colors(
                    focusedContainerColor = Color.Black.copy(alpha = 0.3f),
                    unfocusedContainerColor = Color.Black.copy(alpha = 0.3f),
                    focusedIndicatorColor = Color.Transparent,
                    unfocusedIndicatorColor = Color.Transparent,
                    focusedTextColor = Color.White,
                    unfocusedTextColor = Color.White
                ),
                shape = RoundedCornerShape(20.dp),
                modifier = Modifier.weight(1f)
            )

            Spacer(modifier = Modifier.width(8.dp))

            IconButton(
                onClick = {
                    if (inputText.isNotBlank()) {
                        messages.add(Message("${messages.size + 1}", inputText, true, "21:31"))
                        inputText = ""
                    } else {
                        // إرسال صوتي
                        messages.add(Message("${messages.size + 1}", "🎤 رسالة صوتية مشفرة (0:04 ثانية)", true, "21:31"))
                    }
                },
                modifier = Modifier
                    .size(40.dp)
                    .background(CyberEmerald, CircleShape)
            ) {
                Icon(
                    imageVector = if (inputText.isBlank()) Icons.Default.Mic else Icons.Default.Send,
                    contentDescription = "Send",
                    tint = Color.Black
                )
            }
        }
    }
}

@Composable
fun AttachmentItem(icon: ImageVector, label: String, color: Color, onClick: () -> Unit) {
    Column(
        horizontalAlignment = Alignment.CenterHorizontally,
        modifier = Modifier.clickable { onClick() }
    ) {
        Box(
            modifier = Modifier
                .size(48.dp)
                .background(color.copy(alpha = 0.2f), CircleShape),
            contentAlignment = Alignment.Center
        ) {
            Icon(icon, contentDescription = label, tint = color, modifier = Modifier.size(24.dp))
        }
        Spacer(modifier = Modifier.height(6.dp))
        Text(label, color = MutedSlate, fontSize = 11.sp)
    }
}
