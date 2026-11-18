import SwiftUI
import LocalAuthentication

struct BiometricLockView: View {
    @Binding var isUnlocked: Bool
    @State private var isAuthenticating = false
    @State private var authError: String?

    var body: some View {
        ZStack {
            AetherTheme.deepObsidian.ignoresSafeArea()

            VStack(spacing: 32) {
                Spacer()

                // Aether Monogram Icon
                ZStack {
                    RoundedRectangle(cornerRadius: 28, style: .continuous)
                        .fill(AetherTheme.cardSurface)
                        .frame(width: 96, height: 96)
                        .overlay(
                            RoundedRectangle(cornerRadius: 28, style: .continuous)
                                .stroke(AetherTheme.cyberEmerald.opacity(0.3), lineWidth: 1.5)
                        )
                        .shadow(color: AetherTheme.cyberEmerald.opacity(0.15), radius: 20)

                    Image(systemName: "shield.lefthalf.filled.badge.checkmark")
                        .font(.system(size: 42, weight: .semibold))
                        .foregroundColor(AetherTheme.cyberEmerald)
                }

                VStack(spacing: 8) {
                    Text("أثيـر | AETHER")
                        .font(.system(size: 26, weight: .bold))
                        .foregroundColor(.white)
                        .tracking(1.5)

                    Text("تشفير هجين • صفر خوادم • صفر هوية")
                        .font(.system(size: 13, weight: .medium))
                        .foregroundColor(AetherTheme.mutedSlate)
                }

                Spacer()

                // Biometric Touch Trigger
                Button(action: authenticateUser) {
                    VStack(spacing: 16) {
                        ZStack {
                            Circle()
                                .fill(AetherTheme.cardSurface)
                                .frame(width: 88, height: 88)
                                .overlay(
                                    Circle()
                                        .stroke(AetherTheme.cyberEmerald, lineWidth: 2)
                                        .scaleEffect(isAuthenticating ? 1.15 : 1.0)
                                        .opacity(isAuthenticating ? 0.3 : 0.8)
                                        .animation(Animation.easeInOut(duration: 1).repeatForever(autoreverses: true), value: isAuthenticating)
                                )

                            Image(systemName: "touchid")
                                .font(.system(size: 44, weight: .light))
                                .foregroundColor(AetherTheme.cyberEmerald)
                        }

                        Text("المس المستشعر لفك تشفير الجلسة محلياً")
                            .font(.system(size: 13))
                            .foregroundColor(AetherTheme.mutedSlate)
                    }
                }
                .disabled(isAuthenticating)

                if let error = authError {
                    Text(error)
                        .font(.system(size: 12))
                        .foregroundColor(.red)
                }

                Spacer()

                Text("RAM Zeroize Protocol • Secure Enclave")
                    .font(.system(size: 11, design: .monospaced))
                    .foregroundColor(AetherTheme.mutedSlate.opacity(0.6))
                    .padding(.bottom, 24)
            }
            .padding(.horizontal, 24)
        }
        .onAppear {
            authenticateUser()
        }
    }

    private func authenticateUser() {
        let context = LAContext()
        var error: NSError?

        if context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error) {
            isAuthenticating = true
            let reason = "فك تشفير مفاتيح الجلسة المؤقتة في الذاكرة"

            context.evaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, localizedReason: reason) { success, _ in
                DispatchQueue.main.async {
                    isAuthenticating = false
                    if success {
                        withAnimation(.spring(response: 0.5, dampingFraction: 0.7)) {
                            isUnlocked = true
                        }
                    } else {
                        authError = "فشل التحقق من الهوية الحيوية"
                    }
                }
            }
        } else {
            // المحاكي أو جهاز لا يدعم البصمة
            withAnimation {
                isUnlocked = true
            }
        }
    }
}
