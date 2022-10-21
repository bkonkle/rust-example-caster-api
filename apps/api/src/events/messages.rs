use fake::Dummy;
use serde::{Deserialize, Serialize};
use warp::ws::Message;

/// Incoming WebSocket messages from clients
#[derive(Clone, Debug, Dummy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum IncomingMessage {
    /// A Ping message, which should echo back a Pong
    Ping,
}

impl IncomingMessage {
    pub fn from_message(msg: Message) -> Result<Option<Self>, serde_json::Error> {
        let msg = if let Ok(s) = msg.to_str() {
            s
        } else {
            return Ok(None);
        };

        serde_json::from_str(msg)
    }
}

/// Outgoing WebSocket messages to clients
#[derive(Clone, Debug, Dummy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum OutgoingMessage {
    /// A Pong message, which is the response to a Ping
    Pong,
}

impl From<OutgoingMessage> for Message {
    fn from(msg: OutgoingMessage) -> Message {
        Message::text(serde_json::to_string(&msg).expect("Unable to serialize OutgointMessage"))
    }
}
