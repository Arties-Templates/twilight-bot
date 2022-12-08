use super::commands::{
    echo::{self, Echo},
    followup::{self, FollowUp},
    hello::{self, Hello},
};
use crate::context::{CommandContext, Context};
use std::{error::Error, sync::Arc};
use twilight_interactions::command::CreateCommand;
use twilight_model::application::{
    command::Command,
    interaction::{Interaction, InteractionData, InteractionType},
};

pub async fn interaction(
    ctx: Context,
    interaction: Interaction,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    match interaction.kind {
        InteractionType::ApplicationCommand => handle_command(&ctx, interaction).await?,
        _ => panic!("Unknown interaction: {interaction:#?}"),
    }

    Ok(())
}

async fn handle_command(
    ctx: &Context,
    interaction: Interaction,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let command_ctx = CommandContext {
        twilight: Arc::clone(ctx),
        interaction_id: interaction.id,
        interaction_token: interaction.token.clone(),
        channel_id: interaction.channel_id.unwrap(),
        guild_id: interaction.guild_id.unwrap(),
        user_id: interaction.author_id().unwrap(),
    };

    let data = if let Some(InteractionData::ApplicationCommand(data)) = interaction.data {
        data
    } else {
        panic!("Command Interaction did not contain any data")
    };

    match data.name.as_str() {
        "hello" => hello::run(&command_ctx).await?,
        "echo" => echo::run(&command_ctx, *data).await?,
        "followup" => followup::run(&command_ctx, *data).await?,
        _ => panic!("Unknown interaction command"),
    };

    Ok(())
}

pub async fn register_commands(ctx: Context) -> Result<(), Box<dyn Error + Send + Sync>> {
    let commands: Vec<Command> = vec![
        Hello::create_command().into(),
        Echo::create_command().into(),
        FollowUp::create_command().into(),
    ];

    let client = ctx.http.interaction(ctx.application_id);

    if let Err(why) = client.set_global_commands(&commands).exec().await {
        panic!("Failed to set global commands {}", why)
    };

    Ok(())
}
