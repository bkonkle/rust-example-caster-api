use hyper::{body::to_bytes, client::HttpConnector, Body, Client as HyperClient, Method, Request};
use hyper_tls::HttpsConnector;
use tokio::sync::RwLock;

use crate::get_addr;
use crate::tests::test_server::Server;

lazy_static! {
    static ref SERVER: RwLock<Server> = RwLock::new(Server::new());
}

async fn init_server() {
    SERVER.write().await.init().await;
}

fn http_client() -> HyperClient<HttpsConnector<HttpConnector>> {
    let https = HttpsConnector::new();
    HyperClient::builder().build::<_, Body>(https)
}

#[tokio::test]
#[ignore]
async fn test_initial() {
    init_server().await;

    let addr = get_addr();
    let http_client = http_client();

    let req = Request::builder()
        .method(Method::POST)
        .uri(format!("http://{addr}", addr = addr))
        .body(Body::empty())
        .unwrap();

    let resp = http_client.request(req).await.unwrap();
    assert_eq!(resp.status(), 200);

    let body_bytes = to_bytes(resp.into_body()).await.unwrap();
    assert_eq!(
        body_bytes,
        r#"{"id":1,"name":"wiremock cat fact","checked":false}"#
    );
}
