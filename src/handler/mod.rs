use std::{collections::HashMap, io::Result};

mod response;
pub use response::*;
use tokio::io::AsyncWriteExt;

mod get;
mod post;

pub async fn get(mut context: super::Context) -> Result<()> {
    let response: Response = match context.query.as_str() {
        "/" => get::index(&mut context).await,
        _ => Response::new(ResponseStatus::NotFound),
    };

    context
        .stream
        .write_all(response.serialize().as_bytes())
        .await?;

    Ok(())
}

pub async fn post(mut context: super::Context) -> std::io::Result<()> {
    let form_data: HashMap<String, String> = context
        .body
        .split('&')
        .filter_map(|pair| {
            let mut kv = pair.splitn(2, '=');
            let key = kv.next()?.to_string();
            let value = kv.next()?.to_string();
            Some((key, value))
        })
        .collect();

    let response: Response = match context.query.as_str() {
        "/hello" => post::hello(&mut context, form_data).await,
        _ => Response::new(ResponseStatus::NotFound),
    };

    context
        .stream
        .write_all(response.serialize().as_bytes())
        .await?;

    Ok(())
}
