use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    net::{TcpListener, TcpStream},
    task,
};

mod handler;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (stream, _) = listener.accept().await?;

        task::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Error: {}", e);
            }
        });
    }
}

pub struct Context {
    pub method: String,
    pub query: String,
    pub stream: TcpStream,
    pub user_agent: String,
    pub body: String,
}
impl Context {
    pub fn new(
        method: String,
        query: String,
        stream: TcpStream,
        user_agent: String,
        body: String,
    ) -> Self {
        Self {
            method,
            query,
            stream,
            user_agent,
            body,
        }
    }
}

async fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).await?;
    let (method, query) = {
        let mut req = request_line.split_whitespace().into_iter();
        (
            req.next().unwrap_or("GET").to_string(),
            req.next().unwrap_or("/").to_string(),
        )
    };
    let mut headers = Vec::new();
    let mut header_line = String::new();
    loop {
        header_line.clear();
        buf_reader.read_line(&mut header_line).await?;
        if header_line.trim().is_empty() {
            break;
        }
        headers.push(header_line.trim().to_string());
    }
    let user_agent = headers
        .iter()
        .find(|h| h.starts_with("User-Agent:"))
        .map(|h| h.splitn(2, ':').nth(1).unwrap_or("").trim())
        .unwrap_or("")
        .to_string();

    let content_length = headers
        .iter()
        .find(|h| h.starts_with("Content-Length:"))
        .and_then(|h| h.splitn(2, ':').nth(1))
        .and_then(|s| s.trim().parse::<usize>().ok())
        .unwrap_or(0);

    let mut body = vec![0; content_length];
    if content_length > 0 {
        buf_reader.read_exact(&mut body).await?;
    }
    let body = String::from_utf8_lossy(&body).to_string();

    let context = Context::new(method, query, stream, user_agent, body);
    match context.method.as_str() {
        "GET" => handler::get(context).await?,
        "POST" => handler::post(context).await?,
        _ => {}
    }
    Ok(())
}
