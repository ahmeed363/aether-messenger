import SwiftUI

struct MessageItem: Identifiable {
    let id = UUID()
    let text: String
    let isOutgoing: Bool
    let time: String
    let isMedia: Bool
}

struct ChatView: View {
    @Binding var isUnlocked: Bool
    @State private var inputText: String = ""
    @State private var showAttachmentSheet: Bool = false
    @State private var dummyPacketCount: Int = 14892
    @State private var messages: [MessageItem] = [
        MessageItem(text: "أهلاً! تم إنشاء الجلسة بمفاتيح هجينة (Kyber + X25519) عبر شبكة Nym.", isOutgoing: false, time: "21:28", isMedia: false),
        MessageItem(text: "صورة مشفرة (1.2 MB) تم تقطيعها إلى حزم Sphinx متساوية.", isOutgoing: true, time: "21:29", isMedia: true)
    ]

    let timer = Timer.publish(every: 2.8, on: .main, in: .common).autoconnect()

    var body: some View {
        ZStack {
            AetherTheme.deepObsidian.ignoresSafeArea()

            VStack(spacing: 0) {
                // 1. Top Bar
                HStack(spacing: 12) {
                    Button(action: {
                        withAnimation { isUnlocked = false }
                    }) {
                        Image(systemName: "chevron.right")
                            .foregroundColor(.white)
                            .font(.system(size: 16, weight: .semibold))
                    }

                    Circle()
                        .fill(AetherTheme.cardSurface)
                        .frame(width: 40, height: 40)
                        .overlay(
                            Text("ع")
                                .foregroundColor(AetherTheme.cyberEmerald)
                                .font(.system(size: 16, weight: .bold))
                        )

                    VStack(alignment: .leading, spacing: 2) {
                        HStack(spacing: 6) {
                            Text("علي (Node #702)")
                                .font(.system(size: 14, weight: .bold))
                                .foregroundColor(.white)

                            Text("PQ-Hybrid")
                                .font(.system(size: 9, design: .monospaced))
                                .padding(.horizontal, 6)
                                .padding(.vertical, 2)
                                .background(AetherTheme.cyberEmerald.opacity(0.15))
                                .foregroundColor(AetherTheme.cyberEmerald)
                                .cornerRadius(4)
                        }

                        HStack(spacing: 4) {
                            Circle()
                                .fill(AetherTheme.cyberEmerald)
                                .frame(width: 6, height: 6)

                            Text("Cover Traffic: 1.2 pkt/s")
                                .font(.system(size: 10, design: .monospaced))
                                .foregroundColor(AetherTheme.mutedSlate)
                        }
                    }

                    Spacer()

                    // Quick Zeroize / Burn Button
                    Button(action: purgeMemory) {
                        Image(systemName: "flame.fill")
                            .foregroundColor(.red.opacity(0.8))
                            .font(.system(size: 17))
                            .padding(8)
                            .background(Color.red.opacity(0.15))
                            .clipShape(Circle())
                    }
                }
                .padding(.horizontal, 16)
                .padding(.vertical, 10)
                .background(AetherTheme.cardSurface)

                // 2. Mixnet Status Line
                HStack {
                    Text("NYM SPHINX:")
                        .foregroundColor(AetherTheme.mutedSlate)
                    Text("حزمة وهمية #\(dummyPacketCount)")
                        .foregroundColor(AetherTheme.cyberEmerald)
                    Spacer()
                    Text("Ratchet #42 • PFS")
                        .foregroundColor(AetherTheme.mutedSlate)
                }
                .font(.system(size: 10, design: .monospaced))
                .padding(.horizontal, 16)
                .padding(.vertical, 4)
                .background(Color.black.opacity(0.4))

                // 3. Messages List
                ScrollViewReader { proxy in
                    ScrollView {
                        LazyVStack(spacing: 12) {
                            ForEach(messages) { msg in
                                HStack {
                                    if msg.isOutgoing { Spacer() }

                                    VStack(alignment: msg.isOutgoing ? .trailing : .leading, spacing: 4) {
                                        if msg.isMedia {
                                            ZStack {
                                                RoundedRectangle(cornerRadius: 12)
                                                    .fill(Color.black.opacity(0.4))
                                                    .frame(width: 200, height: 110)

                                                VStack(spacing: 6) {
                                                    Image(systemName: "photo.fill")
                                                        .foregroundColor(AetherTheme.cyberEmerald)
                                                        .font(.system(size: 28))
                                                    Text(msg.text)
                                                        .font(.system(size: 11))
                                                        .multilineTextAlignment(.center)
                                                        .foregroundColor(.white)
                                                }
                                                .padding(8)
                                            }
                                        } else {
                                            Text(msg.text)
                                                .font(.system(size: 13))
                                                .foregroundColor(.white)
                                                .padding(.horizontal, 14)
                                                .padding(.vertical, 10)
                                                .background(msg.isOutgoing ? AetherTheme.outgoingBubble : AetherTheme.incomingBubble)
                                                .cornerRadius(18)
                                        }

                                        Text(msg.time + (msg.isOutgoing ? " • تم التسليم ✓✓" : ""))
                                            .font(.system(size: 9, design: .monospaced))
                                            .foregroundColor(msg.isOutgoing ? AetherTheme.cyberEmerald : AetherTheme.mutedSlate)
                                            .padding(.horizontal, 4)
                                    }

                                    if !msg.isOutgoing { Spacer() }
                                }
                            }
                        }
                        .padding(16)
                    }
                }

                // 4. Attachments Drawer (WhatsApp style)
                if showAttachmentSheet {
                    HStack(spacing: 20) {
                        AttachmentButton(icon: "photo.on.rectangle", label: "معرض الصور", color: AetherTheme.cyberEmerald) {
                            sendMediaMessage(type: "صورة عالية الدقة مشفرة")
                        }
                        AttachmentButton(icon: "doc.fill", label: "مستند", color: AetherTheme.quantumCyan) {
                            sendMediaMessage(type: "مستند PDF مشفر")
                        }
                        AttachmentButton(icon: "camera.fill", label: "كاميرا", color: .indigo) {
                            sendMediaMessage(type: "لقطة كاميرا فورية")
                        }
                        AttachmentButton(icon: "waveform", label: "صوت", color: .orange) {
                            sendMediaMessage(type: "مقطع صوتي مشفر")
                        }
                    }
                    .padding(.vertical, 14)
                    .frame(maxWidth: .infinity)
                    .background(AetherTheme.cardSurface)
                    .transition(.move(edge: .bottom).combined(with: .opacity))
                }

                // 5. WhatsApp-Style Input Bar
                HStack(spacing: 10) {
                    Button(action: {
                        withAnimation(.spring()) {
                            showAttachmentSheet.toggle()
                        }
                    }) {
                        Image(systemName: "plus")
                            .font(.system(size: 18, weight: .bold))
                            .foregroundColor(showAttachmentSheet ? AetherTheme.cyberEmerald : .white)
                            .rotationEffect(.degrees(showAttachmentSheet ? 45 : 0))
                            .frame(width: 38, height: 38)
                            .background(AetherTheme.cardSurface)
                            .clipShape(Circle())
                    }

                    HStack {
                        TextField("رسالة مشفرة بنظام Sphinx...", text: $inputText)
                            .font(.system(size: 14))
                            .foregroundColor(.white)
                            .accentColor(AetherTheme.cyberEmerald)
                    }
                    .padding(.horizontal, 14)
                    .padding(.vertical, 9)
                    .background(Color.black.opacity(0.3))
                    .cornerRadius(20)

                    Button(action: handleSend) {
                        Image(systemName: inputText.isEmpty ? "mic.fill" : "arrow.up")
                            .font(.system(size: 16, weight: .bold))
                            .foregroundColor(.black)
                            .frame(width: 40, height: 40)
                            .background(AetherTheme.cyberEmerald)
                            .clipShape(Circle())
                            .shadow(color: AetherTheme.cyberEmerald.opacity(0.3), radius: 8)
                    }
                }
                .padding(.horizontal, 12)
                .padding(.vertical, 8)
                .background(AetherTheme.cardSurface)
            }
        }
        .onReceive(timer) { _ in
            dummyPacketCount += 1
        }
    }

    private func handleSend() {
        guard !inputText.isEmpty else {
            // Voice note simulation
            sendMediaMessage(type: "🎤 رسالة صوتية مشفرة (0:04)")
            return
        }

        messages.append(MessageItem(text: inputText, isOutgoing: true, time: "21:30", isMedia: false))
        inputText = ""
    }

    private func sendMediaMessage(type: String) {
        showAttachmentSheet = false
        messages.append(MessageItem(text: type, isOutgoing: true, time: "21:30", isMedia: true))
    }

    private func purgeMemory() {
        withAnimation {
            messages.removeAll()
            isUnlocked = false
        }
    }
}

struct AttachmentButton: View {
    let icon: String
    let label: String
    let color: Color
    let action: () -> Void

    var body: some View {
        Button(action: action) {
            VStack(spacing: 6) {
                ZStack {
                    Circle()
                        .fill(color.opacity(0.2))
                        .frame(width: 48, height: 48)

                    Image(systemName: icon)
                        .foregroundColor(color)
                        .font(.system(size: 20))
                }

                Text(label)
                    .font(.system(size: 11))
                    .foregroundColor(AetherTheme.mutedSlate)
            }
        }
    }
}
