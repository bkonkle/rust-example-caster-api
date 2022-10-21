use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    RwLock,
};
use ulid::Ulid;
use warp::ws::Message;

#[derive(Default)]
pub struct Connections(Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>);

impl Connections {
    pub async fn send(&self, conn_id: &str, message: Message) {
        if let Some(connection) = self.0.read().await.get(conn_id) {
            if let Err(_disconnected) = connection.send(message) {
                // The tx is disconnected
            }
        }
    }

    pub async fn insert(&self, tx: UnboundedSender<Message>) -> String {
        let conn_id = Ulid::new().to_string();

        self.0.write().await.insert(conn_id.clone(), tx);

        conn_id
    }

    pub async fn remove(&self, conn_id: &str) {
        self.0.write().await.remove(conn_id);
    }
}
