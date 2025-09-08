use blockchain_project::web_server::WebServer;

fn main() {
    println!("🚀 Запуск веб-сервера Food Truck Network...");
    
    let server = WebServer::new(8080);
    server.start();
}
