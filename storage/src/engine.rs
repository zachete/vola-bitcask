use crate::{Record, Scanner};

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
}
