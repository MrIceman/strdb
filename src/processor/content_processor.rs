use crate::processor::io::file_writer::{FileWriter, init_file_writer};
use crate::processor::model::entry::StrdbEntry;

pub struct Processor<'a> {
    path: &'a str,
    pub file_writer: FileWriter<'a>,
}

pub fn init(path: &str) -> Processor {
    return Processor {
        path,
        file_writer: init_file_writer(path),
    }
}

impl Processor<'_> {
    pub fn write_something(self: &Self, content: &'_ str) -> StrdbEntry {
        let bytes = content.as_bytes();
        let entry = StrdbEntry {
            length: bytes.len() as u16,
            content: content.as_bytes().to_vec(),
        };
      //  self.file_writer.process_entry(&entry);
        entry
    }
}


