use crate::{
    InitializedServer, HttpMethod, Server
};
use colored::*;
use regex::*;
use std::io::{Read, Write};

impl Server {
    pub fn new() -> Self {
        Self {
            ip_addr: String::from("127.0.0.1"),
            port: String::from("8080"),
        }
    }

    pub fn set_ip_port(mut self, ip: String, port: String) -> Self {
        self.ip_addr = ip;
        self.port = port;
        self
    }

    pub fn init_server(self) -> Result<InitializedServer, Box<dyn std::error::Error>> {
        println!("{}", "
┳┓┳┳┏┓┏┳┓  ┏┓┏┓┳┓┓┏┏┓┳┓
┣┫┃┃┗┓ ┃   ┗┓┣ ┣┫┃┃┣ ┣┫
┛┗┗┛┗┛ ┻   ┗┛┗┛┛┗┗┛┗┛┛┗
                       
".green());

        let ip = self.ip_addr.as_str();
        println!("{}{}", "[+] ip : ".green(), ip);
        let port = self.port.as_str();
        println!("{}{}", "[+] port : ".green(), port);

        let cfg_server = match std::net::TcpListener::bind(format!("{}:{}", ip, port)) {
            Ok(listener) => {
                println!("\n🦀 {}{}:{}","http://".yellow(), ip.yellow(), port.yellow());
                listener
            },
            Err(e) => {
                eprintln!("Une erreur s'est produite: {}", e);
                return Err(Box::new(e));
            }
        };

        for income in cfg_server.incoming() {
            if let Ok(mut conn) = income {
                let mut buff: [u8; 1024 * 2] = [0; 1024 * 2];

                match conn.read(&mut buff) {
                    Ok(_bytes_read) => {
                        let client_data = String::from_utf8_lossy(&buff[.._bytes_read]);
                        println!("\n{}", client_data);

                        let check_methode = || -> HttpMethod {
                            let get_method = Regex::new(r"GET\s/").unwrap();
                            let post_method = Regex::new(r"POST\s/").unwrap();

                            if get_method.is_match(&client_data) {
                                return HttpMethod::Get;
                            } else if post_method.is_match(&client_data) {
                                return HttpMethod::Post;
                            }
                            HttpMethod::NotAllowed
                        };

                        let file_request = || -> Option<&str> {
                            let request_data = Regex::new(r"(GET|POST)\s(/[\w\-\.]*)?").unwrap();
                            if let Some(cap) = request_data.captures(&client_data) {
                                if let Some(filename) = cap.get(2) {
                                    return Some(filename.as_str());
                                }
                            }
                            None
                        };

                        let _requested_file = file_request().unwrap_or("/");

                        let response = match check_methode() {
                            HttpMethod::Get => {
                                let file_path = match _requested_file {
                                    "/" => "index.html",
                                    _ => &_requested_file[1..], // Ignorer le premier "/"
                                };

                                match std::fs::read_to_string(file_path) {
                                    Ok(file_content) => {
                                        format!(
                                            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                                            file_content.len(),
                                            file_content
                                        )
                                    }
                                    Err(_) => {
                                        "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nFile Not Found".to_string()
                                    }
                                }
                            }
                            HttpMethod::Post => {
                                format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nPOST request received: {}",
                                    _requested_file
                                )
                            }
                            HttpMethod::NotAllowed => {
                                "HTTP/1.1 405 Method Not Allowed\r\nContent-Type: text/plain\r\n\r\nMethod Not Allowed".to_string()
                            }
                        };

                        conn.write(response.as_bytes()).unwrap();

                    }
                    Err(e) => {
                        eprintln!("Une erreur s'est produite: {}", e);
                        return Err(Box::new(e));
                    }
                }
            }
        }

        Ok(InitializedServer {})
    }
}