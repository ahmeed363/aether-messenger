import asyncio
import json
import os
from websockets.asyncio.server import serve

WEB_DIR = os.path.join(os.path.dirname(__file__), "web")
INDEX_PATH = os.path.join(WEB_DIR, "index.html")

connected_clients = {
    "web": set(),
    "android": set(),
}

def get_index_html():
    if os.path.exists(INDEX_PATH):
        with open(INDEX_PATH, "r", encoding="utf-8") as f:
            return f.read()
    return "<h1>Aether Web Server Ready</h1>"

def process_request(connection, request):
    # If this is a WebSocket upgrade request, let websockets handle it
    if request.headers.get("Upgrade", "").lower() == "websocket":
        return None

    # Normal browser HTTP request -> serve the Aether Web App HTML
    html_str = get_index_html()
    resp = connection.respond(200, html_str)
    resp.headers["Content-Type"] = "text/html; charset=utf-8"
    resp.headers["Access-Control-Allow-Origin"] = "*"
    resp.headers["Cache-Control"] = "no-cache"
    return resp

async def broadcast_device_status():
    status_msg = json.dumps({
        "type": "device_status",
        "androidConnected": len(connected_clients["android"]) > 0,
        "webConnected": len(connected_clients["web"]) > 0,
    })
    for ws in list(connected_clients["web"]):
        try:
            await ws.send(status_msg)
        except Exception:
            pass

async def handle_connection(websocket):
    client_type = "unknown"
    try:
        async for message in websocket:
            try:
                data = json.loads(message)
                msg_type = data.get("type")

                if msg_type == "register":
                    client_type = data.get("client", "web")
                    if client_type in connected_clients:
                        connected_clients[client_type].add(websocket)
                        print(f"[+] Client Registered: {client_type.upper()} ({len(connected_clients[client_type])} active)")
                        await broadcast_device_status()

                elif msg_type == "message":
                    sender = data.get("sender", client_type)
                    text = data.get("text", "")
                    print(f"[*] Message from [{sender.upper()}]: {text}")

                    # Relay to opposite client
                    target_group = "android" if sender == "web" else "web"
                    relay_payload = json.dumps(data)

                    for target_ws in list(connected_clients[target_group]):
                        try:
                            await target_ws.send(relay_payload)
                        except Exception as e:
                            print(f"Error forwarding to {target_group}: {e}")

            except json.JSONDecodeError:
                pass
    finally:
        if client_type in connected_clients and websocket in connected_clients[client_type]:
            connected_clients[client_type].remove(websocket)
            print(f"[-] Client Disconnected: {client_type.upper()}")
            await broadcast_device_status()

async def main():
    host = "0.0.0.0"
    port = 8080
    print("=" * 60)
    print("   🌐 Aether Quantum Messenger — Live Relay & Web Server")
    print(f"   Local PC Web:     http://localhost:{port}/")
    print(f"   Phone Relay URL:  ws://192.168.1.253:{port}/ws")
    print("=" * 60)

    server = await serve(handle_connection, host, port, process_request=process_request)
    await server.serve_forever()

if __name__ == "__main__":
    try:
        asyncio.run(main())
    except KeyboardInterrupt:
        print("\n[!] Aether Server stopped.")
