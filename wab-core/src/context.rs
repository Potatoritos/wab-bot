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
    pub async fn respond(&self, data: InteractionResponseData) {
        let response = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(data)
        };
        let result = self.client.http
            .interaction(self.interaction.application_id.clone())
            .create_response(self.interaction.id.clone(), &self.interaction.token, &response)
            .await;
    }
    pub async fn delete_response(&self) {
        let result = self.client.http
            .interaction(self.interaction.application_id.clone())
            .delete_response(&self.interaction.token)
            .await;
        if !result.as_ref().is_ok_and(|x| x.status().is_success()) {
            tracing::error!("failed to delete response: {result:?}");
        }
    }

}
