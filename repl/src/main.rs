use ::storage::Engine;
use storage::Input;

use std::io::{Write, stdin, stdout};

fn main() -> std::io::Result<()> {
    let mut engine = Engine::new("db");
    engine.scan()?;

    loop {
        let mut prompt = String::new();
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut prompt).expect("can't read line");
        let input = Input::new(prompt.trim_end().to_string());
        let output = engine.try_run(input);
        println!("{}", output);
    }
}
