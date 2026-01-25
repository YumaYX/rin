# rr

Run a command repeatedly with a fixed interval.

`rr` is a small CLI tool written in Rust that executes a given command multiple times, optionally waiting a specified number of seconds between runs.

---

## Features

- Run any command repeatedly
- Specify number of runs with `-n`
- Specify interval (in seconds) with `-i` (supports sub-second values)
- Pass commands and arguments as-is
- Sensible defaults (3 run, 1 second interval)

---

## Installation

### From source

```sh
git clone https://github.com/YumaYX/rr.git
cd rr
cargo build --release
```

```sh
cargo install --path .
```

The binary will be located at:

```sh
target/release/rr
```

Move it somewhere in your `PATH` if needed.

---

## Usage

```sh
rr [OPTIONS] <COMMAND>...
```

### Options

- `-n, --num <N>`  
  Number of times to run the command  
  Default: `1`

- `-i, --interval <SECONDS>`  
  Interval between runs in seconds (floating point allowed)  
  Default: `1.0`

---

## Examples

Run a command once (default behavior):

```sh
rr echo hello
```

Run a command 5 times with a 1 second interval:

```sh
rr -n 5 echo hello
```

Run a command 10 times with a 0.5 second interval:

```sh
rr -n 10 -i 0.5 curl http://localhost
```

Run without waiting between executions:

```sh
rr -n 5 -i 0 make test
```

---

## Notes

- Commands are executed sequentially.
- If a command exits with a non-zero status, execution stops.
- The interval is not applied after the final run.

---

## Motivation

Shell loops and ad-hoc retry logic get tedious fast.

`rr` makes repeated execution simple and predictable:

```sh
rr -n 20 -i 1 some-command
```

Short name, fast typing, no surprises.

