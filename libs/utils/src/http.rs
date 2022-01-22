use hyper::{client::HttpConnector, Body, Client};
use hyper_tls::HttpsConnector;

/// Creates an http/https client via Hyper
pub fn http_client() -> Client<HttpsConnector<HttpConnector>> {
    Client::builder().build::<_, Body>(HttpsConnector::new())
}
