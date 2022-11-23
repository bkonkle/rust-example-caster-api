//! # Generate the schema.graphql file and save it in the current directory
#![forbid(unsafe_code)]

use anyhow::Result;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use caster_api::{graphql::create_schema, Context};
use caster_utils::config::get_config;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    let config = get_config();
    let context = Arc::new(Context::init(config).await?);

    let schema = create_schema(context)?;

    let mut file = File::create("schema.graphql").await?;
    file.write_all(schema.sdl().as_bytes()).await?;

    println!("\n>> Schema saved to schema.graphql\n");

    Ok(())
}
