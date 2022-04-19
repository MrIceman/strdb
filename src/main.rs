use std::borrow::{Borrow, BorrowMut};
use futures::executor::block_on;

use crate::processor::content_processor::init;
use crate::processor::model::config::{Config, get_config};

mod processor;

fn main() {
    let mut config = get_config();
    config.describe();

    async {
        config.init().await
    };

    config.describe();

    let fp = init("hello.txt");
    fp.write_something("Hello World!");
}


struct Test {
    content: String,
    age: u8
}
