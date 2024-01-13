use crate::{Client, State};
use std::sync::Arc;
use twilight_model::{
    application::interaction::Interaction,
    http::interaction::{
        InteractionResponse,
        InteractionResponseType,
        InteractionResponseData
    },
};
use twilight_util::builder::InteractionResponseDataBuilder;
use twilight_http::response::Response;

pub struct EventContext {
    pub state: Arc<State>,
}
pub struct CommandContext {
    pub state: Arc<State>,
    pub client: Arc<Client>,
    pub interaction: Interaction,
}
impl CommandContext {
    pub async fn respond(&self, build: impl FnOnce(InteractionResponseDataBuilder) -> InteractionResponseDataBuilder) {
        let response = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(build(InteractionResponseDataBuilder::new()).build())
        };
        let _result = self.client.http
            .interaction(self.interaction.application_id.clone())
            .create_response(self.interaction.id.clone(), &self.interaction.token, &response)
            .await;
    }
}
