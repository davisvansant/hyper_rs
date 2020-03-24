use hyper::{Body, Method, Request, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://httpbin.org/post")
        .header("content-type", "applicaton/json")
        .body(Body::from(r#"{"library": "hyper"}"#))?;

        let client = Client::new();
        let resp = client.request(req).await?;

        println!("Response: {:?}", resp.status());

        Ok(())
}
