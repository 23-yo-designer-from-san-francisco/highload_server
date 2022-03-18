use highload_server::ThreadPool;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use num_cpus;
use lazy_static::lazy_static;
use regex::{Regex};
use String;
use urlencoding::decode;
use std::fs::{metadata};
use config::{Config, FileFormat, File, Format};
use std::fs;
use std::env;

static SERVER_NAME: &str = "rust";

fn main() {
    let builder = Config::builder()
    .set_default("cpu_limit", "4").unwrap()
    .set_default("host", "0.0.0.0:7878").unwrap()
    .set_default("document_root", "/var/www/html").unwrap()
    .add_source(File::new("/etc/httpd.conf", FileFormat::Toml));

    match builder.build()  {
        Ok(config) => {
            let host: &str = config.get("host").unwrap();
            println!("Host {}", host);
            let listener = TcpListener::bind(host).unwrap();
            let cpus_str: &str = config.get("cpu_limit").unwrap();
            let mut cpus: usize = cpus_str.parse().unwrap();
            let actual_cpus = num_cpus::get();
            if cpus <= 0 || cpus > actual_cpus {
                cpus = actual_cpus;
            }
            let pool = ThreadPool::new(cpus);
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let path = config.get("document_root").unwrap();
                pool.execute(move || {
                    handle_connection(stream, path);
                });
            }
        
            println!("Shutting down.");
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}

fn handle_connection(mut stream: TcpStream, base_path: &str) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    lazy_static! {
        static ref REQUEST_RE: Regex = Regex::new(r#"(GET|HEAD|POST|PUT|OPTIONS|DELETE|CONNECT|TRACE|PATCH) (/[\w./\-?=%&]*) HTTP/1\.(0|1)"#).unwrap();
    }

    let request = String::from_utf8_lossy(&buffer);

    let req = REQUEST_RE.captures(&request);

    if let Some(matches) = req {
        let method = matches.get(1).map_or("GET", |m| m.as_str());
 
         match method {
             "GET" | "HEAD" => {
                let path = matches.get(2).map_or("/", |m| m.as_str()).to_string();
                let mut full_path: String = base_path.to_owned();
        
                let response: String;

                full_path.push_str(&path);
                let mut full_path = decode(&full_path).unwrap().to_string().replace("../", "");
                
                lazy_static! {
                    static ref REQUEST_PARAMS_RE: Regex = Regex::new(r#"[?&][\w]+=[\w]+"#).unwrap();
                }

                full_path = REQUEST_PARAMS_RE.replace_all(&full_path, "").to_string();

                let mut is_dir = false;
                if let Ok(meta) = metadata(&full_path) {
                    if meta.is_dir() {
                        is_dir = true;
                        full_path.push_str("index.html");
                    }
                }

                match fs::read(&full_path) {
                    Ok(contents) => {
                        
                        let content_type: &str;
                        if let Some(ext) = Path::new(&full_path).extension().and_then(|s| s.to_str()) {
                            content_type = match ext {
                                "html" => "text/html",
                                "css" => "text/css",
                                "js" => "application/javascript",
                                "jpg" => "image/jpeg",
                                "jpeg" => "image/jpeg",
                                "png" => "image/png",
                                "gif" => "image/gif",
                                "swf" => "application/x-shockwave-flash",
                                _=>  "text/plain",
                            };
                        } else {
                            content_type = "text/plain";
                        }
        
                        let status_line = "HTTP/1.1 200 OK";
                        let dt = chrono::offset::Utc::now();
                        response = format!(
                            "{}\r\nDate: {}\r\nServer: {}\r\nContent-Length: {}\r\nConnection: {}\r\nContent-type: {}\r\n\r\n",
                            status_line,
                            dt.to_rfc2822().replace("+0000", "GMT"),
                            SERVER_NAME,
                            contents.len(),
                            "close",
                            content_type,
                        );
                        stream.write(response.as_bytes()).unwrap();
                        if method == "GET" {
                            stream.write(&contents).unwrap_or_default();
                        }
                    },
                    Err(_) => {
                        response = match is_dir {
                            true =>  "HTTP/1.1 403 Forbidden".to_string(),
                            false => "HTTP/1.1 404 Not Found".to_string(),
                        };
                        let response = format!("{}\r\nServer: {}", response, SERVER_NAME);
                        stream.write(response.as_bytes()).unwrap();
                    }   
                }
             },
             "POST"|"PUT"|"OPTIONS"|"DELETE"|"CONNECT"|"TRACE"|"PATCH" => {
                let response = "HTTP/1.1 405 Method Not Allowed\r\nAllow: GET, HEAD".to_string();
                stream.write(response.as_bytes()).unwrap();
             }
             _ => {},
         }
    
        stream.flush().unwrap();
    }
}
