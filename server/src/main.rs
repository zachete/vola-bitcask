use std::{
    io::{Read, Result, Write},
    net::TcpListener,
};
use storage::Engine;

fn main() -> Result<()> {
    let mut engine = Engine::new("db");
    engine.scan()?;

    let listener = TcpListener::bind("127.0.0.1:1666")?;

    for maybe_stream in listener.incoming() {
        let mut stream = maybe_stream?;

        loop {
            let mut buffer = [0; 64];
            stream.read(&mut buffer)?;
            let chars = str::from_utf8(&buffer).unwrap().chars();
            let mut command = String::new();
            for chr in chars {
                if chr == '\n' {
                    break;
                }

                command.push(chr);
            }

            let mut parsed = command.splitn(3, ' ');
            match parsed.next() {
                Some("get") => {
                    let key = parsed.next().unwrap();
                    let maybe_value = engine.get(key);
                    match maybe_value {
                        Some(val) => {
                            let mut buf = Vec::new();
                            let prefix = key.as_bytes();
                            buf.extend_from_slice(prefix);
                            buf.push('=' as u8);
                            let mut value = val.value;
                            buf.append(&mut value);
                            buf.push('\n' as u8);
                            stream.write_all(&buf)?;
                        }
                        None => {
                            let bytes = "NOT FOUND\n".as_bytes();
                            stream.write(bytes)?;
                        }
                    }
                }
                Some("set") => {
                    let key = parsed.next().unwrap();
                    let value = parsed.next().unwrap();
                    engine.set(key, value)?;
                    stream.write("OK\n".as_bytes())?;
                }
                Some("scan") => {
                    engine.scan()?;
                }
                None => {}
                _ => {}
            }
        }
    }

    Ok(())
}
