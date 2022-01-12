# Minimal reproducible example for my Rust Issue from 2022-01 about Rustc/LLVM

https://github.com/rust-lang/rust/issues/92760

This example shows that the problem that crashes Rustc/LLVM is not in `hasbrown` but 
in fact somewhere in `fontdue`.

See `main.rs`/`entry_rust()` for problematic code.

Build with `§ cargo build`. It will output: \
`LLVM ERROR: Do not know how to split the result of this operator!`