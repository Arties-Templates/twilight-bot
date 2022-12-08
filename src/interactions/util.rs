#![allow(dead_code)]

use std::error::Error;
use twilight_http::{response::marker::EmptyBody, Response};
use twilight_model::{
    channel::{embed::Embed, Message},
    http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
};

use crate::context::CommandContext;

/// Follow up to a interaction with a text response
pub async fn follow_up_text(
    ctx: &CommandContext,
    message: &str,
) -> Result<Response<Message>, Box<dyn Error + Send + Sync>> {
    let application_id = ctx.twilight.application_id;
    let interaction_token = ctx.interaction_token.as_ref();

    let msg = ctx
        .twilight
        .http
        .interaction(application_id)
        .create_followup(interaction_token)
        .content(message)?
        .exec()
        .await?;

    Ok(msg)
}
/// Reply to a interaction with a custom response
pub async fn reply(
    ctx: &CommandContext,
    response: InteractionResponseData,
) -> Result<Response<EmptyBody>, Box<dyn Error + Send + Sync>> {
    let interaction_id = ctx.interaction_id;
    let application_id = ctx.twilight.application_id;
    let interaction_token = ctx.interaction_token.as_ref();

    let msg = ctx
        .twilight
        .http
        .interaction(application_id)
        .create_response(
            interaction_id,
            interaction_token,
            &InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(response),
            },
        )
        .exec()
        .await?;

    Ok(msg)
}

/// Edit a interaction replys' text
pub async fn edit_reply_text(
    ctx: &CommandContext,
    new_data: &str,
) -> Result<Response<Message>, Box<dyn Error + Send + Sync>> {
    let application_id = ctx.twilight.application_id;
    let interaction_token = ctx.interaction_token.as_ref();

    let msg = ctx
        .twilight
        .http
        .interaction(application_id)
        .update_response(interaction_token)
        .content(Some(new_data))?
        .exec()
        .await?;

    Ok(msg)
}

/// Edit a interaction replys' text
pub async fn edit_reply_embed(
    ctx: &CommandContext,
    new_data: Embed,
) -> Result<Response<Message>, Box<dyn Error + Send + Sync>> {
    let application_id = ctx.twilight.application_id;
    let interaction_token = ctx.interaction_token.as_ref();

    let msg = ctx
        .twilight
        .http
        .interaction(application_id)
        .update_response(interaction_token)
        .embeds(Some(&[new_data]))?
        .exec()
        .await?;

    Ok(msg)
}
