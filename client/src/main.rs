use hyper::*;
use hyper::Client;
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};
// use tokio::io::* as _;
// use tokio::*;
//
// Box<dyn std::error::Error + Send + Sync>'

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    let uri = "http://httpbin.org/ip".parse::<Uri>().unwrap();
    let mut resp = client.get(uri).await?;

    println!("Response: {:?}", resp.status());

    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await.unwrap();
    }

    Ok(())
}
