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
}
impl Data {
    pub fn new(method: String, query: String, stream: TcpStream) -> Self {
        Self {
            method,
            query,
            stream,
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let (method, query) = get_request_info(&http_request);
    let data = Data::new(method, query, stream);
    match data.method.as_str() {
        "GET" => handler::get(data).await,
        "POST" => {}
        _ => {}
    }
    println!("{:#?}", http_request);
    Ok(())
}

fn get_request_info(http_request: &Vec<String>) -> (String, String) {
    let arr: Vec<String> = http_request
        .get(0)
        .unwrap()
        .split(" ")
        .map(|c| c.to_string())
        .collect();
    return (arr.get(0).unwrap().clone(), arr.get(1).unwrap().clone());
}
