use std::borrow::Borrow;
use std::fs::File;
use std::io::{Read, Write};

use crate::processor::model::entry::StrdbEntry;

pub struct FileWriter<'a> {
    pub path: &'a str,
    pub entry_amount: i64,
}

pub fn init_file_writer(path: &str) -> FileWriter {
    let fw = FileWriter {
        path,
        entry_amount: -1,
    };
    fw.init();
    fw
}

impl FileWriter<'_> {
    pub fn init(self: &Self) {
        println!("initializing");
        let file = File::open(&self.path).unwrap();
        let mut file_copy = file.try_clone().unwrap();
        let mut buf = [0; 4];
        file_copy.read_exact(&mut buf);
        let entries_amount = u32::from_be_bytes(buf);
        println!("{:?}", entries_amount);
        if entries_amount == 0 {
            file_copy.write(String::from("hello world").as_bytes());
        }
        self.process_entry(&StrdbEntry {
            length: 80,
            content: String::from("this is a test").into_bytes(),
        })
    }

    pub fn process_entry(self: &Self, entry: &StrdbEntry) {
        let file = File::create(&self.path);
        file.unwrap().write(entry.content.borrow());
    }
}
