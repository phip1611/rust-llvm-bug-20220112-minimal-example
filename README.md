# Minimal reproducible example for my Rust Issue from 2022-01 about Rustc/LLVM

https://github.com/rust-lang/rust/issues/92760

This example shows that the problem that crashes Rustc/LLVM is not in `hasbrown` but 
in fact somewhere in `fontdue`.

See `main.rs`/`entry_rust()` for problematic code.

Build with `§ cargo build`. It will output: \
`LLVM ERROR: Do not know how to split the result of this operator!`

## Update #1
- after disabling the `simd`-feature of `fontdue` the LLVM error message is gone
  and the error message is now:

```text
-20220112-minimal-example/target/x86_64-none-bare_metal/debug/deps/kernel_bin-b2e3b701572fd441" "--gc-sections"
  = note: rust-lld: error: undefined symbol: fmaxf
          >>> referenced by f32.rs:690 (/home/pschuster/.rustup/toolchains/nightly-2022-01-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:690)
          >>>               /home/pschuster/dev/other/rust-llvm-bug-20220112-minimal-example/target/x86_64-none-bare_metal/debug/deps/kernel_bin-b2e3b701572fd441.4i8fwqz53syl3m5f.rcgu.o:(core::f32::_$LT$impl$u20$f32$GT$::max::h3b280365286bb855)
          >>> referenced by f32.rs:690 (/home/pschuster/.rustup/toolchains/nightly-2022-01-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:690)
          >>>               ttf_parser-6b7cd33f39fc07ca.ttf_parser.7b1c2514-cgu.7.rcgu.o:(core::f32::_$LT$impl$u20$f32$GT$::max::h8cf801bec5e7b913) in archive /home/pschuster/dev/other/rust-llvm-bug-20220112-minimal-example/target/x86_64-none-bare_metal/debug/deps/libttf_parser-6b7cd33f39fc07ca.rlib
          
          rust-lld: error: undefined symbol: fminf
          >>> referenced by f32.rs:709 (/home/pschuster/.rustup/toolchains/nightly-2022-01-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:709)
          >>>               /home/pschuster/dev/other/rust-llvm-bug-20220112-minimal-example/target/x86_64-none-bare_metal/debug/deps/kernel_bin-b2e3b701572fd441.4i8fwqz53syl3m5f.rcgu.o:(core::f32::_$LT$impl$u20$f32$GT$::min::h7f29acd76dc9436e)
          >>> referenced by f32.rs:709 (/home/pschuster/.rustup/toolchains/nightly-2022-01-03-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/num/f32.rs:709)
          >>>               ttf_parser-6b7cd33f39fc07ca.ttf_parser.7b1c2514-cgu.7.rcgu.o:(core::f32::_$LT$impl$u20$f32$GT$::min::h47ed670ba851f6e4) in archive /home/pschuster/dev/other/rust-llvm-bug-20220112-minimal-example/target/x86_64-none-bare_metal/debug/deps/libttf_parser-6b7cd33f39fc07ca.rlib

```