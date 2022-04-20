use std::borrow::{Borrow, BorrowMut};

use crate::processor::manager::init;
use crate::processor::model::config::{get_config};

mod processor;

fn main() {
    let mut config = get_config();
    config.describe();

    async {
        config.init().await;
    };

    config.describe();

    let fp = init("hello.txt", &config, );
    fp.write_something("Hello World!");
}


struct Test {
    content: String,
    age: u8
}
