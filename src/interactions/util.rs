use std::error::Error;
use twilight_http::{Response, response::marker::EmptyBody};
use twilight_model::{http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
}, channel::Message};

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

    let msg = ctx.twilight
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
