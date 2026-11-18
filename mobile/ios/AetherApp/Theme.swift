import SwiftUI

public struct AetherTheme {
    public static let deepObsidian = Color(hex: "0B0E14")
    public static let cyberEmerald = Color(hex: "00E599")
    public static let quantumCyan  = Color(hex: "00D2D3")
    public static let incomingBubble = Color(hex: "181D26")
    public static let outgoingBubble = Color(hex: "0F3327")
    public static let cardSurface  = Color(hex: "0E131C")
    public static let mutedSlate   = Color(hex: "64748B")
}

extension Color {
    init(hex: String) {
        let scanner = Scanner(string: hex)
        var rgbValue: UInt64 = 0
        scanner.scanHexInt64(&rgbValue)
        let r = Double((rgbValue & 0xFF0000) >> 16) / 255.0
        let g = Double((rgbValue & 0x00FF00) >> 8) / 255.0
        let b = Double(rgbValue & 0x0000FF) / 255.0
        self.init(red: r, green: g, blue: b)
    }
}
