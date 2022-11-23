use axum::extract::ws::WebSocket;
use futures::{SinkExt, StreamExt, TryFutureExt};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{events::router::route_message, Context};
use caster_auth::authenticate::Subject;

use super::messages::IncomingMessage;

/// Handle `WebSocket` connections by setting up a message handler that deserializes them and
/// determines how to handle
pub async fn handle(socket: WebSocket, ctx: Arc<Context>, _sub: Subject) {
    let (mut ws_write, mut ws_read) = socket.split();

    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            ws_write
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    let conn_id = ctx.connections.insert(tx).await;

    while let Some(result) = ws_read.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", conn_id, e);
                break;
            }
        };

        match IncomingMessage::from_message(msg) {
            Ok(Some(message)) => route_message(ctx.clone(), &conn_id, message).await,
            Ok(None) => {
                // pass
            }
            Err(err) => {
                eprintln!("json error(uid={}): {}", conn_id, err);
            }
        }
    }

    eprintln!("good bye user: {}", conn_id);

    ctx.connections.remove(&conn_id).await;
}
