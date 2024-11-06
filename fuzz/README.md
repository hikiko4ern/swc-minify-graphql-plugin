# swc-plugin-minify-graphql-fuzz

To perform fuzz testing:

1. build the AFL LLVM runtime (only needed once for each Rust version used):
   ```sh
   cargo bin cargo-afl config --build
   ```
2. build the target:
   ```sh
   cargo bin cargo-afl build --release --bin minify
   ```
3. start the fuzzer:
   ```sh
   cargo bin cargo-afl fuzz -i afl_in -o afl_out target/release/minify
   ```

See the [Rust Fuzz Book](https://rust-fuzz.github.io/book/afl/tutorial.html#build-the-fuzz-target) and the [AFL++ documentation](https://github.com/AFLplusplus/AFLplusplus/tree/v4.21c/docs) for more details.
