use std::sync::Arc;

use super::messages::{
    IncomingMessage::{self, Ping},
    OutgoingMessage::Pong,
};
use crate::Context;

/// Route `WebSocket` messages to handlers
pub async fn route_message(ctx: Arc<Context>, conn_id: &str, message: IncomingMessage) {
    match message {
        Ping => handle_ping(&ctx, conn_id).await,
    }
}

async fn handle_ping(ctx: &Arc<Context>, conn_id: &str) {
    ctx.connections.send(conn_id, Pong.into()).await;
}
