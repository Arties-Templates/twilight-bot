use super::commands::{
    echo::{self, Echo},
    hello::{self, Hello},
};
use crate::context::{CommandContext, Context};
use std::{error::Error, sync::Arc};
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::{
        command::Command,
        interaction::{Interaction, InteractionData, InteractionType},
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
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
    let client = ctx.http.interaction(ctx.application_id);
    let command_ctx = CommandContext {
        twilight: Arc::clone(ctx),
        interaction_id: interaction.id,
    };

    let data = if let Some(InteractionData::ApplicationCommand(data)) = interaction.data {
        data
    } else {
        panic!("Command Interaction did not contain any data")
    };

    let response = match data.name.as_str() {
        "hello" => hello::run(command_ctx)?,
        "echo" => echo::run(command_ctx, *data)?,
        _ => panic!("Unknown interaction command"),
    };

    client
        .create_response(
            interaction.id,
            &interaction.token,
            &InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(response),
            },
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn register_commands(ctx: Context) -> Result<(), Box<dyn Error + Send + Sync>> {
    let commands: Vec<Command> = vec![
        Hello::create_command().into(),
        Echo::create_command().into(),
    ];

    let client = ctx.http.interaction(ctx.application_id);

    if let Err(why) = client.set_global_commands(&commands).exec().await {
        panic!("Failed to set global commands {}", why)
    };

    Ok(())
}
