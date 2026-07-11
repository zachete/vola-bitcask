# vola-kv

A tiny Rust implementation of a [Bitcask](https://riak.com/assets/bitcask-intro.pdf)-style
key-value store, built for learning purposes.

## What this is

Bitcask is a log-structured storage engine: writes are appended to a log file,
and an in-memory index maps each key to its latest position on disk. This
project is a minimal, educational implementation — not production-ready.

## Features

- `set <key> <value>` — append a write to the log and update the index
- `get <key>` — look up a key via the in-memory index and read its value from disk
- `scan` — iterate over all keys currently in the index
- In-memory index (key → file offset) rebuilt/maintained as writes happen

## Not implemented (yet)

- Compaction / merging of old log segments
- Deletes (tombstones)
- Concurrent access

## Usage

There are two ways to run it: as a server or in REPL mode.

### Server mode

Start the server:
```bash
cargo run -p server
```

Connect and send commands using netcat:
```bash
➜ nc localhost 1666
set a 1
OK
set b 2
OK
get a
a=1
get b
b=2
```

### REPL mode

```bash
➜ cargo run -p repl

> set a 1
OK
> set b 2
OK
> get a
a=1
> get b
b=2
```
