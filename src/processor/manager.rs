use std::borrow::Borrow;

use crate::processor::io::engine::StrDbEngine;
use crate::processor::io::file_writer::{FileWriter, init_file_writer};
use crate::processor::io::write_worker::WriteWorker;
use crate::processor::io::writer::write_worker_impl::create_write_worker;
use crate::processor::model::config::Config;
use crate::processor::model::entry::StrdbEntry;

pub struct Manager<'a> {
    path: &'a str,
    config: &'a Config,
    write_workers: &'a [&'a dyn WriteWorker],
}

pub fn init<'a>(path: &'a str, config: &'a Config, engine: &'static dyn StrDbEngine) -> Box<Manager<'a>> {
    let mut workers: Vec<&'a dyn WriteWorker> = Vec::new();
    for i in 0..config.write_workers_amount {
        workers.push(
            &*create_write_worker(
                i.to_string().as_str(),
                engine,
            ),
        );
    }
    return Box::new(Manager {
        path,
        config,
        write_workers: workers.as_slice(),
    })
}

impl Manager<'_> {
    pub fn write_something(self: &Self, content: &'_ str) -> StrdbEntry {
        let bytes = content.as_bytes();
        let entry = StrdbEntry {
            length: bytes.len() as u16,
            content: content.as_bytes().to_vec(),
        };

        entry
    }
}


