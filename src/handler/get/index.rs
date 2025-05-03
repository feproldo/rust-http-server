use crate::handler::{Response, ResponseStatus};

pub async fn index(_context: &mut crate::Context) -> Response {
    Response::new(ResponseStatus::Ok).content(format!(
        include_str!("../../../html/index.html"),
        format!("<style>{}</style>", include_str!("../../../css/main.css")),
        include_str!("../../../html/components/header.html")
    ))
}
