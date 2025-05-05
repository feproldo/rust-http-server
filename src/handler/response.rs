pub enum ResponseStatus {
    Ok,
    NotFound,
    NoContent,
    InternalServerError,
    Created,
}

impl ResponseStatus {
    fn serialize(&self) -> String {
        String::from(match self {
            Self::Ok => "200 OK",
            Self::NotFound => "404 NOT FOUND",
            Self::NoContent => "400 NO CONTENT",
            Self::InternalServerError => "500 INTERNAL SERVER ERROR",
            Self::Created => "201 CREATED",
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
            format!(
                "HTTP/1.1 {}\r\n\
                Content-Type: text/html; charset=utf-8\r\n\
                Content-Length: {}\r\n\
                \r\n\
                {}",
                self.status.serialize(),
                content.len(),
                content
            )
        } else {
            return format!("HTTP/1.1 {}\r\n\r\n", self.status.serialize());
        }
    }
}
