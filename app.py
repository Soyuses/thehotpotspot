# -*- coding: utf-8 -*-
import os
import json
from http.server import HTTPServer, SimpleHTTPRequestHandler
from urllib.parse import urlparse
import time

class FoodTruckHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=os.path.dirname(os.path.abspath(__file__)), **kwargs)
    
    def do_GET(self):
        parsed_path = urlparse(self.path)
        if parsed_path.path.startswith("/api/"):
            self.handle_api_request(parsed_path)
        else:
            super().do_GET()
    
    def handle_api_request(self, parsed_path):
        path = parsed_path.path
        if path == "/api/status":
            self.send_json_response({
                "status": "online",
                "service": "Food Truck Network",
                "version": "2.0",
                "timestamp": time.time()
            })
        else:
            self.send_error(404, "API endpoint not found")
    
    def send_json_response(self, data):
        self.send_response(200)
        self.send_header("Content-type", "application/json")
        self.send_header("Access-Control-Allow-Origin", "*")
        self.end_headers()
        response = json.dumps(data, indent=2)
        self.wfile.write(response.encode("utf-8"))

def run_server(port=8000):
    server_address = ("", port)
    httpd = HTTPServer(server_address, FoodTruckHandler)
    print(f"🚀 Food Truck Network Server запущен на порту {port}")
    print(f"🌐 Веб-интерфейс: http://localhost:{port}")
    print(f"📱 Мобильное приложение: http://localhost:{port}/mobile_app/")
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n🛑 Сервер остановлен")
        httpd.shutdown()

if __name__ == "__main__":
    port = int(os.environ.get("PORT", 8000))
    run_server(port)