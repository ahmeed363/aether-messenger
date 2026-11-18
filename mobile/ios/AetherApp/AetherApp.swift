import SwiftUI

@main
struct AetherApp: App {
    @State private var isUnlocked: Bool = false

    var body: some Scene {
        WindowGroup {
            if isUnlocked {
                ChatView(isUnlocked: $isUnlocked)
                    .transition(.opacity)
            } else {
                BiometricLockView(isUnlocked: $isUnlocked)
                    .transition(.opacity)
            }
        }
    }
}
