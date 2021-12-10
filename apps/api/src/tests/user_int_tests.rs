use std::{net::SocketAddr, time::Duration};

use hyper::{body::to_bytes, client::HttpConnector, Body, Client as HyperClient, Method, Request};
use hyper_tls::HttpsConnector;
use tokio::time::sleep;

use crate::run;

async fn init_server() -> SocketAddr {
    let (addr, server) = run(0).await;

    // Spawn the server in the background
    tokio::spawn(server);

    // Wait for it to initialize
    sleep(Duration::from_millis(200)).await;

    // Return the bound address
    addr
}

fn http_client() -> HyperClient<HttpsConnector<HttpConnector>> {
    let https = HttpsConnector::new();
    HyperClient::builder().build::<_, Body>(https)
}

#[tokio::test]
#[ignore]
async fn test_initial() {
    let addr = init_server().await;

    let http_client = http_client();

    let req = Request::builder()
        .method(Method::POST)
        .uri(format!("http://localhost:{port}/", port = addr.port()))
        .body(Body::from("{}"))
        .unwrap();

    let resp = http_client.request(req).await.unwrap();
    assert_eq!(resp.status(), 200);

    let body_bytes = to_bytes(resp.into_body()).await.unwrap();
    assert_eq!(body_bytes, r#"{"data":null}"#);
}
