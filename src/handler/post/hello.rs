use std::collections::HashMap;

use crate::handler::Response;

pub async fn hello(_ctx: &mut crate::Context, form_data: HashMap<String, String>) -> Response {
    Response::new(crate::handler::ResponseStatus::Ok).content(format!(
        include_str!("../../../html/hello.html"),
        format!("<style>{}</style>", include_str!("../../../css/main.css")),
        include_str!("../../../html/components/header.html"),
        form_data.get("name").unwrap_or(&"user".to_string())
    ))
}
