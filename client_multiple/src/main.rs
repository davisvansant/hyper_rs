use hyper::{Client, Uri};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let ip_fut = async {
        let resp = client.get(Uri::from_static("http://httpbin.org/ip")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };

    let headers_fut = async {
        let resp = client.get(Uri::from_static("http://httpbin.org/headers")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };

    let (ip, headers) = futures::try_join!(ip_fut, headers_fut)?;

    println!("Ip: {:?}", ip);
    println!("Headers: {:?}", headers);

    Ok(())
}
