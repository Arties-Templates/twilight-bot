use std::error::Error;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::application_command::CommandData;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{context::CommandContext, interactions::util::reply};

#[derive(CommandModel, CreateCommand)]
#[command(name = "echo", desc = "echo, echo, echo, echo.")]
pub struct Echo {
    #[command(desc = "The text to echo")]
    text: String,
}

pub async fn run(
    ctx: &CommandContext,
    command_data: CommandData,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let options = Echo::from_interaction(command_data.into())?;

    let response = InteractionResponseDataBuilder::new()
        .content(options.text)
        .build();

    reply(ctx, response).await?;

    Ok(())
}
