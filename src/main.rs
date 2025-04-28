use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

mod handler;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream).await?;
    }
    Ok(())
}

pub struct Data {
    pub method: String,
    pub query: String,
    pub stream: TcpStream,
    pub user_agent: String
}
impl Data {
    pub fn new(method: String, query: String, stream: TcpStream, user_agent: String) -> Self {
        Self {
            method,
            query,
            stream,
            user_agent
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf_reader = BufReader::new(&stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line)?;
    let (method, query) = {
        let mut req = request_line.split_whitespace().into_iter();
        (req.next().unwrap_or("GET").to_string(), req.next().unwrap_or("/").to_string())
    };
    let mut headers = Vec::new();
    let mut header_line = String::new();
    
    loop {
        buf_reader.read_line(&mut header_line)?;
        if header_line.trim().is_empty() {
            break;
        }
        headers.push(header_line.trim().to_string());
    }
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let user_agent = headers.iter()
        .find(|h| h.starts_with("User-Agent:"))
        .map(|h| h.splitn(2, ':').nth(1).unwrap_or("").trim())
        .unwrap_or("")
        .to_string();
    let data = Data::new(method, query, stream, user_agent);
    match data.method.as_str() {
        "GET" => handler::get(data).await,
        "POST" => {}
        _ => {}
    }
    Ok(())
}