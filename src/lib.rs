use std::io::{Read, Write};
use regex::{*};

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
        /*init server */
        let ip = self.ip_addr.as_str();
        let port = self.port.as_str();

        let cfg_server = match std::net::TcpListener::bind(format!("{}:{}", ip, port)) {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Une erreur s'est prduite: {}", e);
                return Err(Box::new(e));
            }
        };

        for income in cfg_server.incoming() {
            if let Ok(mut conn) = income {
                let mut buff: [u8; 1024 * 2] = [0; 1024 * 2];

                match conn.read(&mut buff) {
                    Ok(_bytes_read) => {
                        let client_data = String::from_utf8_lossy(&buff[.._bytes_read]);
                        println!("{}", client_data);

                        // Vérification de la méthode HTTP (GET ou POST)
                        let check_methode = || -> HttpMethod {
                            let get_method = Regex::new(r"GET\s/").unwrap();
                            let post_method = Regex::new(r"POST\s/").unwrap();

                            if get_method.is_match(&client_data) {
                                return HttpMethod::Get;
                            } else if post_method.is_match(&client_data) {
                                return HttpMethod::Post;
                            }
                            HttpMethod::NotAllowed // méthode non autorisée
                        };

                        // Extraction du fichier demandé
                        let file_request = || -> Option<&str> {
                            let request_data = Regex::new(r"(GET|POST)\s(/[\w\-\.]*)?").unwrap();
                            if let Some(cap) = request_data.captures(&client_data) {
                                // On capture spécifiquement le chemin (groupe 2)
                                if let Some(filename) = cap.get(2) {
                                    return Some(filename.as_str());
                                }
                            }
                            None
                        };

                        // Utilisation de la méthode HTTP et du fichier demandé
                        let _requested_file = file_request().unwrap_or("/");
                        
                        //how send
                        if let HttpMethod::Get = check_methode() {
                            println!("GET: {}", _requested_file);
                            let file = std::fs::read_to_string("index.html").unwrap();
                            println!("{}", file);
                        
                        } else if let HttpMethod::Post = check_methode() {
                            println!("POST: {}", _requested_file);
                        
                        } else {
                            println!("Méthode non autorisée.");
                        }

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
