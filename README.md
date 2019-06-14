# Simple backtracking (bruteforce) sudoku solver written to teach myself Rust

## Build
Use `cargo` to build the application.

```shell
cargo build
```

## Run
To run solver you need to provide it with the task (sudoku) to solve. Task needs to be written in a text file (see examples in in the [tasks folder](./tasks/2.sudoku)). `0` values represent empty cells which need to be solved.
Once you have a task in the file run the app and specify the path to your task file with `-s` option:

```shell
cargo run --release -- -s tasks/2.sudoku
```
