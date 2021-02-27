use anyhow::{anyhow, Result as AnyResult};
use hyper::{body::HttpBody, Body, Client, Response};

#[tokio::main]
async fn main() -> AnyResult<()> {
    println!("my ip is {}", get_my_ip().await?);

    Ok(())
}

async fn get_my_ip() -> AnyResult<String> {
    let client = Client::new();
    let resp = client.get("http://ipinfo.io/ip".parse()?).await?;
    let status = resp.status();

    let body = body_to_string(resp).await?;

    status
        .is_success()
        .then(|| body)
        .ok_or(anyhow!("server response status not successful"))
}

async fn body_to_string(mut resp: Response<Body>) -> AnyResult<String> {
    let mut buf = Vec::new();
    while let Some(chunk) = resp.body_mut().data().await {
        buf.append(&mut chunk?.to_vec());
    }
    Ok(String::from_utf8_lossy(buf.as_slice()).to_string())
}
