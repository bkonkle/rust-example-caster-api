use hyper::{body::to_bytes, Body, Method, Request};

use crate::tests::utils::{http_client, run_server};

#[tokio::test]
#[ignore]
async fn test_initial() {
    let addr = run_server().await;

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
