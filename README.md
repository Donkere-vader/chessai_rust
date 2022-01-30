# ChessAI

This is the chess algorithm for my High school thesis (profielwerkstuk in dutch).

## Build status

| main | [![Rust](https://github.com/Donkere-vader/chessai_rust/actions/workflows/rust.yml/badge.svg)](https://github.com/Donkere-vader/chessai_rust/actions/workflows/rust.yml) |
|---|---|

## Run it

The easiest way is to [build it](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html#building-and-running-a-cargo-project) (in [release mode](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html#building-for-release)) and interact with it via [cutechess](https://github.com/cutechess/cutechess). Because the chess algorithm supports [UCI](https://nl.wikipedia.org/wiki/Universal_Chess_Interface).

When running from cutechess, make sure that the working directory is the project root, and that the command is:

```sh
./target/release/chess_ai UCI
```
