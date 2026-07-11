use std::{
    io::{BufRead, BufReader, Result, Write},
    net::TcpListener,
};
use storage::{Engine, Input};

fn main() -> Result<()> {
    let mut engine = Engine::new("db");
    engine.scan()?;

    let listener = TcpListener::bind("127.0.0.1:1666")?;
    for maybe_stream in listener.incoming() {
        let stream = maybe_stream?;
        let mut reader = BufReader::new(&stream);
        let mut prompt = String::new();

        loop {
            prompt.clear();

            let bytes_read = reader.read_line(&mut prompt)?;
            if bytes_read == 0 {
                break;
            }

            let input = Input::new(prompt.trim().to_string());
            let output = engine.try_run(input);
            write!(&stream, "{}\n", output)?;
        }
    }

    Ok(())
}
