use highload_server::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use num_cpus;
use lazy_static::lazy_static;
use regex::{Regex};
use String;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(num_cpus::get());

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    lazy_static! {
        static ref REQUEST_RE: Regex = Regex::new(r#"(GET|HEAD|POST|PUT|OPTIONS|DELETE|CONNECT|TRACE|PATCH) (/[\w./-]*) HTTP/1\.1"#).unwrap();
    }

    let request = String::from_utf8_lossy(&buffer);
    // println!("{}", request);

    let req = REQUEST_RE.captures(&request);//.len();//.get(0).map_or("", |m| m.as_str());

    if let Some(matches) = req {
        let req_type = matches.get(1).map_or("GET", |m| m.as_str());
        let mut path = matches.get(2).map_or("/", |m| m.as_str());
        // println!("Req type: {}\nPath: {}", req_type, path);

        // let (status_line, filename) = if buffer.starts_with(get) {
        //     ("HTTP/1.1 200 OK", "index.html")
        // } else {
        //     ("HTTP/1.1 404 NOT FOUND", "404.html")
        // };
    
        let base_path = "/Users/l.belyaev/highload_server";

        let mut full_path: String = base_path.to_owned();

        let response: String;
        if path == "/" {
            path = "/index.html";
        }
        full_path.push_str(path);
        match fs::read_to_string(&full_path) {
            Ok(contents) => {
                let extension = Path::new(&full_path).extension().and_then(|s| s.to_str()).unwrap();
                let content_type = match extension {
                    "html" => "text/html",
                    "css" => "text/css",
                    "js" => "text/javascript",
                    "jpg" => "image/jpeg",
                    "jpeg" => "image/jpeg",
                    "png" => "image/png",
                    "gif" => "image/gif",
                    "swf" => "application/x-shockwave-flash",
                    _=>  "text/plain",
                };

                let status_line = "HTTP/1.1 200 OK";
                response = format!(
                    "{}\r\nDate: {}\r\nServer: {}\r\nContent-Length: {}\r\nConnection: {}\r\nContent-type: {}\r\n\r\n{}",
                    status_line,
                    "Today",
                    "rust",
                    contents.len(),
                    "close",
                    content_type,
                    contents
                );
            },
            Err(_) => {
                response = "HTTP/1.1 404 NOT FOUND".to_string();
            }   
        }
    
    
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        // println!("Parse error");
    }
}
