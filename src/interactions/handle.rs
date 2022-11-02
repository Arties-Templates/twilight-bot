use super::commands::hello;
use crate::context::Context;
use std::error::Error;
use tracing::error;
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

    let data = if let Some(InteractionData::ApplicationCommand(data)) = interaction.data {
        data
    } else {
        panic!("Command Interaction did not contain any data")
    };

    let response = match data.name.as_str() {
        "hello" => hello::run(ctx)?,
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
    let commands: Vec<Command> = vec![];

    let client = ctx.http.interaction(ctx.application_id);

    if let Err(why) = client.set_global_commands(&commands).exec().await {
        error!("Failed to set global commands {}", why)
    };

    Ok(())
}
