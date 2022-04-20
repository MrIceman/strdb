use std::borrow::Borrow;
use std::fs::File;

use crate::processor::io::engine::StrDbEngine;
use crate::processor::io::write_worker::WriteWorker;

struct WriteWorkerImpl<'a> {
    uuid: String,
    engine: &'a dyn StrDbEngine,
}

pub fn create_write_worker<'a>(uuid: &str,
                           engine: &'static dyn StrDbEngine) -> Box<dyn WriteWorker> {
    return Box::new(WriteWorkerImpl {
        uuid: String::from(uuid),
        engine,
    })
}

impl WriteWorker for WriteWorkerImpl<'_> {
    fn write(self: &Self, content: &[u8]) {}

    fn spawn(self: &Self) {
        todo!()
    }
}

impl WriteWorkerImpl<'static> {
    fn get_file(self: &Self) -> File {
        let mut path = "./temp/{}".to_string();
        path.push_str(self.uuid.as_str());
        let file = File::open(path.clone());
        match file {
            Ok(_) => {
                file.unwrap()
            }
            Err(_) => {
                File::create(String::from(path.clone())).unwrap()
            }
        }
    }
}
