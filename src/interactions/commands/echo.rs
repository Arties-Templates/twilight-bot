use std::error::Error;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::application_command::CommandData,
    http::interaction::InteractionResponseData,
    id::{marker::InteractionMarker, Id},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::context::Context;

#[derive(CommandModel, CreateCommand)]
#[command(name = "echo", desc = "echo, echo, echo, echo.")]
pub struct Echo {
    #[command(desc = "The text to echo")]
    text: String,
}

pub fn run(
    _interaction_id: Id<InteractionMarker>,
    _ctx: &Context,
    command_data: CommandData,
) -> Result<InteractionResponseData, Box<dyn Error + Send + Sync>> {
    let options = Echo::from_interaction(command_data.into())?;

    let response = InteractionResponseDataBuilder::new()
        .content(options.text)
        .build();

    Ok(response)
}
