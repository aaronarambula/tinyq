# tinyq

A tiny persistent key-value store written in Rust. Think of it as a toy version of Redis — commands are typed into a REPL, data survives restarts via an append-only log.

## What it does

- `set <key> <value>` — store a key-value pair (persists to disk)
- `get <key>` — look up a value by key

Data is written to `db.txt` in the current directory. On startup, tinyq replays the log to rebuild its in-memory state, so your data sticks around between sessions.

## Usage

```
$ cargo run
tinyq > set name aaron
tinyq > get name
aaron
tinyq > exit

$ cargo run
tinyq > get name
aaron
```

## How it works

tinyq uses an **append-only log** — the same pattern behind Redis AOF and PostgreSQL's WAL. Every `set` appends a line to `db.txt`. On startup, tinyq reads the file top-to-bottom and replays each entry into an in-memory `HashMap`. Later writes to the same key overwrite earlier ones, so the final replayed state matches the most recent value.

```
db.txt              →   in-memory HashMap
----                    ------------------
foo bar                 foo → baz
name aaron       →      name → aaron
foo baz
```

## Build

```
cargo run
```

Requires Rust (stable).

## Limitations

This is a learning project, not production software. Known rough edges:

- Crashes on malformed input (e.g. `set` with no args)
- Values can't contain spaces
- The log grows forever — no compaction
- No `delete` command yet
- Single-threaded, no network layer

## Roadmap

- [ ] Graceful error handling for bad commands
- [ ] `delete` via tombstone markers
- [ ] Multi-word values
- [ ] Log compaction
- [X] `exit` command

README.md written by Claude
