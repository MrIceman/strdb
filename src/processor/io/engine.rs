use crate::processor::io::write_message::WriteMessage;
use async_trait::async_trait;

#[async_trait]
pub trait StrDbEngine {
    async fn on_new_message(self: &Self, message: &WriteMessage);
}
