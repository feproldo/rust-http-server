use std::io::Write;

mod get;

pub async fn get(mut data: super::Data) {
    let response: Response = match data.query.as_str() {
        "/main" => get::main(&mut data).await,
        _ => Response::new(ResponseStatus::NotFound),
    };

    data.stream
        .write_all(response.serialize().as_bytes())
        .unwrap();
}

pub enum ResponseStatus {
    Ok,
    NotFound,
    NoContent,
    InternalServerError,
}

impl ResponseStatus {
    fn serialize(&self) -> String {
        String::from(match self {
            Self::Ok => "200 OK",
            Self::NotFound => "404 NOT FOUND",
            Self::NoContent => "400 NO CONTENT",
            Self::InternalServerError => "500 INTERNAL SERVER ERROR",
        })
    }
}

pub struct Response {
    pub status: ResponseStatus,
    pub content: Option<String>,
}

impl Response {
    pub fn new(status: ResponseStatus) -> Self {
        Self {
            status,
            content: None,
        }
    }

    pub fn content<T: Into<String>>(mut self, content: T) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn serialize(self) -> String {
        if let Some(content) = self.content {
            let content: String = content.into();
            return format!(
                "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
                self.status.serialize(),
                content.len(),
                content
            );
        } else {
            return format!("HTTP/1.1 {}\r\n\r\n", self.status.serialize());
        }
    }
}
