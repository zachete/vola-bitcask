use ::storage::Engine;

use std::io::{Write, stdin, stdout};
use std::str;

fn main() -> std::io::Result<()> {
    let mut engine = Engine::new("db");
    engine.scan()?;

    loop {
        let mut prompt = String::new();
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut prompt).expect("can't read line");
        let parsed_prompt: Vec<&str> = prompt.trim_end().split_whitespace().collect();
        let command = parsed_prompt[0];

        match command {
            "set" => {
                let key = parsed_prompt[1];
                let value = parsed_prompt[2];
                engine.set(key, value)?;
                println!("OK");
            }
            "get" => {
                let key = parsed_prompt[1];
                let value = engine.get(key);

                match value {
                    Some(val) => {
                        println!("{}={}", key, str::from_utf8(&val.value).unwrap());
                    }
                    None => {
                        println!("NOT FOUND")
                    }
                }
            }
            "scan" => {
                engine.scan()?;
            }
            _ => {}
        }
    }
}
