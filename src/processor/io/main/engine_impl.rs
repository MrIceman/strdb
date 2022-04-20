use crate::processor::io::engine::StrDbEngine;
use crate::processor::io::write_message::WriteMessage;
use crate::processor::model::config::Config;

struct EngineImpl<'a> {
    config: &'a Config,
}

pub fn init_engine<'a>(config: &'a Config) -> Box<dyn StrDbEngine> {
    return Box::new(EngineImpl { config });
}

#[async_trait]
impl StrDbEngine for EngineImpl {
    async fn on_new_message(&self, message: &WriteMessage) {
        // TODO
        println!("a new message was written to the engine, will write to main db. Just not yet ")
    }
}
