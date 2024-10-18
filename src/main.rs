use simple_server::Server;
fn main() {
    let _server_test = Server::new()
        .set_ip_port(String::from("127.0.0.1"), String::from("9090"))
        .init_server();
}
