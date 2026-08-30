use reqwest::{Error, Response, get};

pub async fn get_request_raw<T: reqwest::IntoUrl>(uri: T) -> Result<Response, Error> {
    return get(uri).await;
}
pub async fn get_request_str<T: reqwest::IntoUrl>(uri: T) -> Result<String, Error> {
    let body = get_request_raw(uri).await?.text().await?;
    return Ok(body)
}
pub async fn get_request_bytes<T: reqwest::IntoUrl>(uri: T) -> Result<Vec<u8>, Error> {
    let body = get_request_raw(uri).await?.bytes().await?;
    return Ok(body.to_vec())
}