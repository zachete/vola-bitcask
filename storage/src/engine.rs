use crate::input::Command;
use crate::{Input, Record, Scanner};

use std::io::SeekFrom;
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Seek},
};

#[derive(Debug)]
struct LogPointer {
    offset: u64,
}

pub struct Engine {
    index: HashMap<Vec<u8>, LogPointer>,
    buf_reader: BufReader<File>,
    buf_writer: BufWriter<File>,
}

impl Engine {
    pub fn new(db_path: &str) -> Self {
        let db_file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .append(true)
            .open(db_path)
            .expect("can't open a file");
        let file_writer_clone = db_file.try_clone().expect("can't clone a file");
        let file_reader_clone = db_file.try_clone().expect("can't clone a file");
        let buf_writer = BufWriter::new(file_writer_clone);
        let buf_reader = BufReader::new(file_reader_clone);

        Self {
            index: HashMap::new(),
            buf_writer,
            buf_reader,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> std::io::Result<()> {
        let mut record = Record::new(key, value);
        let offset = record.write_to(&mut self.buf_writer)?;
        self.index.insert(record.key, LogPointer { offset });
        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Option<Record> {
        let log_pointer = self.index.get(key.as_bytes());
        if let Some(val) = log_pointer {
            self.buf_reader
                .seek(SeekFrom::Start(val.offset))
                .expect("Can't seek reader position");

            let maybe_record =
                Record::read_from(&mut self.buf_reader).map_or(None, |val| Some(val));
            return maybe_record;
        }

        None
    }

    pub fn scan(&mut self) -> std::io::Result<()> {
        Scanner::scan(&mut self.buf_reader, |record, offset| {
            self.index.insert(record.key, LogPointer { offset });
        })?;
        println!("Index: {:?}", self.index);

        Ok(())
    }

    pub fn try_run(&mut self, input: Input) -> String {
        match input.command {
            Command::GET => {
                if let Some(arg) = input.args[0].clone() {
                    let maybe_record = self.get(&arg);
                    if let Some(record) = maybe_record {
                        return String::from_utf8(record.value).unwrap_or("".to_string());
                    }
                }
            }
            Command::SET => {
                let key = match input.args[0].clone() {
                    Some(val) => val,
                    None => {
                        return String::from("Invalid key arg");
                    }
                };
                let value = match input.args[1].clone() {
                    Some(val) => val,
                    None => {
                        return String::from("Invalid value arg");
                    }
                };

                match self.set(&key, &value) {
                    Ok(_) => {
                        return String::from("OK");
                    }
                    Err(_) => return String::from("Can't set the key"),
                }
            }
            Command::NONE => {}
        };

        "".to_string()
    }
}
