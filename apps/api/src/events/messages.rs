use axum::extract::ws::Message;
use fake::Dummy;
use serde::{Deserialize, Serialize};

/// Incoming `WebSocket` messages from clients
#[derive(Clone, Debug, Dummy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum IncomingMessage {
    /// A Ping message, which should echo back a Pong
    Ping,
}

impl IncomingMessage {
    /// Create a new `IncomingMessage` from a `WebSocket` Message
    pub fn from_message(msg: Message) -> Result<Option<Self>, serde_json::Error> {
        let msg = if let Ok(message) = msg.to_text() {
            message
        } else {
            return Ok(None);
        };

        serde_json::from_str(msg)
    }
}

/// Outgoing `WebSocket` messages to clients
#[derive(Clone, Debug, Dummy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum OutgoingMessage {
    /// A Pong message, which is the response to a Ping
    Pong,
}

impl From<OutgoingMessage> for Message {
    fn from(msg: OutgoingMessage) -> Message {
        Message::Text(serde_json::to_string(&msg).expect("Unable to serialize OutgointMessage"))
    }
}
