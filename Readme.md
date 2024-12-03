### Rust-simple-server
![Rust](https://img.shields.io/badge/made%20with-Rust-red)
![License](https://img.shields.io/github/license/joaoviictorti/RustRedOps)
</br>

A simple example of a web server made in Rust

**How to use**
```rust
use simple_server::Server;
fn main() {
    let server_test = Server::new()
        .set_ip_port(String::from("127.0.0.1"), String::from("9090"))
        .init_server();
}

```
