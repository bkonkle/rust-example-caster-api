use futures::{SinkExt, StreamExt, TryFutureExt};
use std::{convert::Infallible, sync::Arc};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws;

use crate::Context;
use caster_auth::authenticate::Subject;

use super::messages::IncomingMessage;

pub async fn handle(
    ws: ws::Ws,
    ctx: Arc<Context>,
    _sub: Subject,
) -> Result<impl warp::Reply, Infallible> {
    let reply = ws.on_upgrade(|socket| async move {
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
    });

    Ok(reply)
}
