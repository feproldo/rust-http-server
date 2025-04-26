use crate::handler::{Response, ResponseStatus};

pub async fn main(data: &mut crate::Data) -> Response {
    Response::new(ResponseStatus::Ok).content(include_str!("../../../html/main.html"))
}
