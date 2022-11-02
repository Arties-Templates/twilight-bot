use std::error::Error;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{context::CommandContext, interactions::util::reply};

#[derive(CommandModel, CreateCommand)]
#[command(name = "hello", desc = "Prints 'Hello World!' in the Chat")]
pub struct Hello;

pub async fn run(ctx: &CommandContext) -> Result<(), Box<dyn Error + Send + Sync>> {
    let response = InteractionResponseDataBuilder::new()
        .content("Hello World!")
        .build();

    reply(ctx, response).await?;

    Ok(())
}
