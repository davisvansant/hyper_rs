use hyper_tls::HttpsConnector;
use hyper::Client;
use serde::{Serialize, Deserialize};
use bytes::buf::ext::BufExt;

#[derive(Serialize, Deserialize, Debug)]
struct Echo {
    args: Args,
    headers: Headers,
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Args {
    foo1: String,
    foo2: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Headers {
    #[serde(rename = "x-forwarded-proto")]
    x_forwarded_proto: Option<String>,
    host: Option<String>,
    accept: Option<String>,
    #[serde(rename = "accept-encoding")]
    accept_encoding: Option<String>,
    #[serde(rename = "cache-control")]
    cache_control: Option<String>,
    #[serde(rename = "postman-token")]
    postman_token: Option<String>,
    #[serde(rename = "user-agent")]
    user_agent: Option<String>,
    #[serde(rename = "x-forwarded-port")]
    x_forwarded_port: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let get_echo = client.get("https://postman-echo.com/get?foo1=bar1&foo2=bar2".parse()?).await?;

    println!("{:?}", get_echo.status());
    println!("{:?}", get_echo.body());

    let echo_body = hyper::body::aggregate(get_echo).await?;
    let body_output: Echo = serde_json::from_reader(echo_body.reader())?;
    println!("{:?}", body_output);

    Ok(())
}
