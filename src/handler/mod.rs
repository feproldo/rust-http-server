use std::io::Write;

mod response;
pub use response::*;

mod get;

pub async fn get(mut data: super::Data) {
    let response: Response = match data.query.as_str() {
        "/main" => get::main(&mut data).await,
        "/get" => get::index(&mut data).await,
        _ => Response::new(ResponseStatus::NotFound),
    };

    data.stream
        .write_all(response.serialize().as_bytes())
        .unwrap();
}