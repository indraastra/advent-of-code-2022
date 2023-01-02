# Solutions

* Days 1 - 5: [python](python/Day%201-5.ipynb)
  * Day 1 [rust](src/bin/day01.rs)
* Day 16: [python](python/Day%2016.ipynb)
* Day 22: [python](python/Day%2022.ipynb)

# Rust

Thanks to [fasterthanli.me](https://fasterthanli.me/series/advent-of-code-2022), whose more-idiomatic Rust solutions were very useful to cross-reference.

Running:
```sh
cargo run --release --bin day01
```

With logging:
```sh
RUST_LOG=info cargo run --bin day01
```

For benchmarking:
```sh
hyperfine ./target/release/day01
```
