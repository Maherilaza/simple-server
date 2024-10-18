use std::io::Read;

#[allow(unused)]
pub struct Server {
    ip_addr : String,
    port : String
}

pub struct InitializedServer {}

impl Server {
    pub fn new() -> Self {
        Self {
            ip_addr: String::from("127.0.0.1"),
            port: String::from("8080")
        }
    }

    pub fn set_ip_port(mut self, ip : String, port : String) -> Self {
        self.ip_addr = ip;
        self.port = port;
        self
    }

    pub fn init_server(self) -> Result<InitializedServer, Box<dyn std::error::Error>> {
        /*init server */
        let ip = self.ip_addr.as_str();
        let port = self.port.as_str();

        let cfg_server =  match std::net::TcpListener::bind(format!("{}:{}", ip, port)) {
            Ok(listener) => listener,
            Err(_) => {
                eprintln!("Une erreur s'est prduite");
                std::process::exit(-1);
            }
        };

        for income in cfg_server.incoming() {
            if let Ok(mut conn) = income {
                let mut buff: [u8; 1024 * 2] = [0; 1024 * 2];

                match conn.read(&mut buff) {
                    Ok(_bytes_read) => {
                        //println!("{}", String::from_utf8_lossy(&buff));
                        let client_data = String::from_utf8_lossy(&buff);
                        println!("{}", client_data);
                    },
                    Err(_) => {
                        eprintln!("Une erreur s'est produite");
                        std::process::exit(-1);
                    }
                }

            }
        }

        Ok(InitializedServer {})
    }
}