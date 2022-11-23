use anyhow::Result;
use futures_util::StreamExt;
use tokio_tungstenite::tungstenite::Message;

use caster_api::events::messages::{IncomingMessage, OutgoingMessage};

mod test_utils;
use test_utils::TestUtils;

#[tokio::test]
#[ignore]
async fn test_ping() -> Result<()> {
    let utils = TestUtils::init().await?;

    let message = serde_json::to_string(&IncomingMessage::Ping)?;

    utils
        .send_message(Message::Text(message), |read| {
            read.take(1).for_each(|message| async {
                let data = message.unwrap().into_data();
                let result = std::str::from_utf8(&data).unwrap();

                let expected = serde_json::to_string(&OutgoingMessage::Pong).unwrap();

                assert_eq!(&expected, result);
            })
        })
        .await?;

    Ok(())
}
