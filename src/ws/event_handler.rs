use super::{models::*};

#[async_trait::async_trait]
pub trait EventHandler {
    async fn on_data(&self, event: WsDataEvent);
}