package network.aether.messenger.net

import android.os.Handler
import android.os.Looper
import okhttp3.*
import org.json.JSONObject
import java.util.concurrent.TimeUnit

class AetherRelayClient(
    private var serverUrl: String = "ws://192.168.1.253:8080/ws",
    private val onMessageReceived: (id: String, text: String, isOutgoing: Boolean, isMedia: Boolean) -> Unit,
    private val onConnectionChanged: (Boolean) -> Unit
) {
    private val client = OkHttpClient.Builder()
        .readTimeout(0, TimeUnit.MILLISECONDS)
        .retryOnConnectionFailure(true)
        .build()

    private var webSocket: WebSocket? = null
    private val mainHandler = Handler(Looper.getMainLooper())
    private var isConnected = false

    fun connect(url: String = serverUrl) {
        serverUrl = url
        disconnect()

        val request = Request.Builder().url(serverUrl).build()
        webSocket = client.newWebSocket(request, object : WebSocketListener() {
            override fun onOpen(webSocket: WebSocket, response: Response) {
                isConnected = true
                mainHandler.post { onConnectionChanged(true) }
                // إرسال رسالة تعريف العقدة للجسر
                val reg = JSONObject().apply {
                    put("type", "register")
                    put("client", "android")
                }
                webSocket.send(reg.toString())
            }

            override fun onMessage(webSocket: WebSocket, text: String) {
                try {
                    val json = JSONObject(text)
                    val type = json.optString("type")
                    if (type == "message") {
                        val sender = json.optString("sender")
                        val content = json.optString("text")
                        val id = json.optString("id", System.currentTimeMillis().toString())
                        val isMedia = json.optBoolean("isMedia", false)
                        
                        // إذا كانت الرسالة واردة من الويب
                        if (sender == "web") {
                            mainHandler.post {
                                onMessageReceived(id, content, false, isMedia)
                            }
                        }
                    }
                } catch (e: Exception) {
                    e.printStackTrace()
                }
            }

            override fun onClosing(webSocket: WebSocket, code: Int, reason: String) {
                webSocket.close(1000, null)
                isConnected = false
                mainHandler.post { onConnectionChanged(false) }
            }

            override fun onFailure(webSocket: WebSocket, t: Throwable, response: Response?) {
                isConnected = false
                mainHandler.post { onConnectionChanged(false) }
                // محاولة إعادة الاتصال التلقائي بعد 3 ثوانٍ
                mainHandler.postDelayed({
                    if (!isConnected) {
                        connect()
                    }
                }, 3000)
            }
        })
    }

    fun sendMessage(content: String, isMedia: Boolean = false) {
        val payload = JSONObject().apply {
            put("type", "message")
            put("sender", "android")
            put("text", content)
            put("isMedia", isMedia)
            put("id", "msg_${System.currentTimeMillis()}")
            put("time", "الآن")
        }
        webSocket?.send(payload.toString())
    }

    fun disconnect() {
        webSocket?.close(1000, "User Left")
        webSocket = null
        isConnected = false
        mainHandler.post { onConnectionChanged(false) }
    }
}
