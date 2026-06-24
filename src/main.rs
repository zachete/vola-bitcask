use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Read, Write, stdout};
use std::{fs::File, io::stdin};

struct Bitcask {
    file: File,
    buf_reader: BufReader<File>,
    buf_writer: BufWriter<File>,
}

impl Bitcask {
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
            file: db_file,
            buf_writer,
            buf_reader,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        writeln!(self.buf_writer, "{}:{}", key, value).expect("can't write to file");
        self.buf_writer.flush().expect("can't flush a buf writer");
        println!("{}:{}", key, value);
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        let reader = self.buf_reader.by_ref();

        for line in reader.lines() {
            if let Ok(val) = line {
                let val_clone = val.clone();
                let parsed_record = val_clone.split(":").collect::<Vec<&str>>();
                if parsed_record[0] == key {
                    return Some(parsed_record[1].to_string());
                }
            }
        }

        None
    }
}

fn main() {
    let mut bitcask = Bitcask::new("db.txt");
    let mut prompt = String::new();

    loop {
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut prompt).expect("can't read line");
        let parsed_prompt: Vec<&str> = prompt.trim_end().split_whitespace().collect();
        let command = parsed_prompt[0];

        match command {
            "set" => {
                let key = parsed_prompt[1];
                let value = parsed_prompt[2];
                bitcask.set(key, value);
            }
            "get" => {
                let key = parsed_prompt[1];
                let value = bitcask.get(key);

                match value {
                    Some(val) => {
                        println!("{}", val);
                    }
                    None => {
                        println!("Not found")
                    }
                }
            }
            _ => {}
        }
    }
}
