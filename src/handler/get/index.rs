use crate::handler::{Response, ResponseStatus};

pub async fn index(data: &mut crate::Data) -> Response {
    Response::new(ResponseStatus::Ok).content(format!(include_str!("../../../html/index.html"), data.user_agent))
}
