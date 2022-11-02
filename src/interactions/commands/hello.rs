use std::error::Error;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::http::interaction::InteractionResponseData;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::context::CommandContext;

#[derive(CommandModel, CreateCommand)]
#[command(name = "hello", desc = "Prints 'Hello World!' in the Chat")]
pub struct Hello;

pub fn run(_ctx: CommandContext) -> Result<InteractionResponseData, Box<dyn Error + Send + Sync>> {
    let response = InteractionResponseDataBuilder::new()
        .content("Hello World!")
        .build();

    Ok(response)
}
