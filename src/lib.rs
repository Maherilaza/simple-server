#[allow(unused)]
pub struct Server {
    ip_addr: String,
    port: String,
}

enum HttpMethod {
    Get,
    Post,
    NotAllowed,
}

pub struct InitializedServer {}
mod init_server;