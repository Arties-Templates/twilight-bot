use crate::{
    context::CommandContext,
    interactions::util::{follow_up_text, reply},
};
use std::error::Error;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::application_command::CommandData;
use twilight_util::builder::InteractionResponseDataBuilder;

#[derive(CommandModel, CreateCommand)]
#[command(name = "followup", desc = "Follows up with a message.")]
pub struct FollowUp {
    #[command(desc = "The text to echo")]
    text: String,
}

pub async fn run(
    ctx: &CommandContext,
    command_data: CommandData,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let options = FollowUp::from_interaction(command_data.into())?;

    let response = InteractionResponseDataBuilder::new()
        .content(options.text)
        .build();

    reply(ctx, response).await?;
    follow_up_text(ctx, "Followed up with this message.").await?;

    Ok(())
}
