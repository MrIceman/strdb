use std::borrow::Borrow;
use std::fs::File;
use std::io::{Read, Write};

pub struct Config {
    amount_of_entries: i64,
    write_workers_amount: u8,
}

pub fn get_config() -> Config {
    return Config {
        amount_of_entries: -999,
        write_workers_amount: 1,
    }
}

impl Config {
    pub fn describe(self: &Self) {
        println!("got {} amount of entries", self.amount_of_entries);
    }

    pub async fn init(self: &mut Self) {
        let file = File::open("./config");
        match file {
            Ok(_) => {
                println!("config file exists already");
                self.set_config(&file.unwrap());
            }
            Err(_) => {
                let f = File::create("./config");
                self.write_config(&f.unwrap());
                println!("created new config file");
            }
        }
    }

    fn set_config(self: &mut Self, file: &File) {
        let mut buf = [0; 8];
        let amount_of_slices = i64::from_be_bytes(buf);
        self.amount_of_entries = amount_of_slices;
    }

    fn write_config(self: &Self, file: &File) {
        let mut file_copy = file.try_clone().unwrap();
        file_copy.write(1_i64.to_be_bytes().as_slice());
    }
}
