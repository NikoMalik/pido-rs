# pido-rs 🚀

**BLAZINGLY FAST AND MEMORY SAFE** library for the legendary `is_even` and `is_odd` functions! 🔥 Check if numbers are even or odd with lightning speed and the ironclad memory safety of Rust! 💾 This crate is a rocket ship ready to blast your projects into performance orbit! 🎉

Welcome to **pido-rs**, where simplicity meets raw power, and even/odd checks become an epic adventure faster than you can say "Rustacean"! 🦀

---

## Why pido-rs? 🌟

- **BLAZINGLY FAST**: Harnesses `rayon` to process millions of numbers in a flash! ⚡
- **MEMORY SAFETY**: Rust ensures your data is safe from memory bugs—no undefined behavior here! 🛡️
- **Legendary Functions**: `is_even` and `is_odd` aren't just checks; they're a masterpiece of computation! 🎨
- **Supercharged Serialization**: Save results with `bincode` faster than the speed of light! 🚀
- **Flexible Power**: Supports any numeric type with `num`, from `i32` to `u64`! 💪
- **CLI AWESOMENESS**: `clap` brings a slick command-line interface to control your number-crunching destiny! 🎮
- **PROGRESS BARS**: `indicatif` adds visual pizzazz to show off your processing in style! 📊
- **EPIC LOGGING**: `tracing` delivers logs so detailed, they’re practically poetry! 📜
- **SERIALIZATION GALORE**: Choose `bincode` for speed or `serde_json` for readable JSON output! 💾
- **ERROR HANDLING PRO**: `anyhow` and `thiserror` make errors bow before your might! 💪


---


## Usage 🎉

**pido-rs** delivers three legendary functions:

- **`is_even<T: Integer + Display>(n: T) -> bool`**: Checks if a number is even. Fast? **BLAZINGLY FAST**! Safe? **MEMORY SAFE**!
- **`is_odd<T: Integer + Display>(n: T) -> bool`**: Checks if a number is odd. Lightning-fast and rock-solid! ⚡
- **`parallel_check(numbers: &[i32]) -> Vec<NumberCheck>`**: Processes millions of numbers in parallel with `rayon`. It's out-of-this-world! 🌌

Example usage in `src/main.rs`:

```rust
use pido_rs::parallel_check;
use log::info;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    env_logger::init();
    let numbers: Vec<i32> = (0..1000000).collect();
    info!("Generated {} numbers", numbers.len());
    let start = std::time::Instant::now();
    let results = parallel_check(&numbers);
    info!("Parallel check completed in {:?}", start.elapsed());
    for result in results.iter().take(5) {
        println!(
            "Number {}: is_even: {}, is_odd: {}",
            result.number, result.is_even, result.is_odd
        );
    }
    let serialized = bincode::serialize(&results).expect("Serialization failed");
    let mut file = File::create("results.bin")?;
    file.write_all(&serialized)?;
    info!("Results saved to results.bin");
    Ok(())
}
```

Run it with the CLI:
```bash
cargo run --release -- --count 1000000 --output results.json --json
```


Try generating fewer numbers or JSON output:
```bash
cargo run --release -- --count 10 --json
```

Or **MAKE BLAZINGLY FAST**
```bash
cargo build --release
./target/release/pido-rs --count 1000000 --output results.bin
```


Run it and feel the **BLAZINGLY FAST** power! 🔥

---

## Testing 🧪

Our tests aren't just tests—they're **BLAZINGLY FAST** validations of awesomeness! 🚀

```bash
cargo test
```

Expected output:
```
running 3 tests
test tests::test_is_even ... ok
test tests::test_is_odd ... ok
test tests::test_parallel_check ... ok
test result: ok. 3 passed; 0 failed; 0 ignored
```

---

## Benchmarks 📈

Want to see how **BLAZINGLY FAST** this crate is? Fire up the benchmarks:

```bash
cargo bench
```

Compare sequential vs. parallel processing—`rayon` makes the parallel version absolutely soar! 🚀

---




## Why Rust? 🦀

Rust isn't just a language—it's a lifestyle! 😎 **pido-rs** taps into Rust's full power for:
- **BLAZINGLY FAST** performance with an optimizing **FASTEST** compiler.
- **MEMORY SAFETY** without a garbage collector—your data is untouchable! 🛡️
- Parallel processing with `rayon` that makes your CPU cores sing! 🎶
- Lightning-fast I/O with `bincode` serialization! ⚡

---

## License 📜

**pido-rs** is released under the BSD2 License. Do whatever you want with it, but keep the **BLAZINGLY FAST** and **MEMORY SAFE** spirit alive! 🔥

---

## Contributing 🤝

Want to make **pido-rs** even faster? Fork the repo, add your magic, and send a pull request! Let's make this project shoot for the stars! 🚀

> **Made with ❤️, Rust, and BLAZINGLY FAST MEMORY SAFETY!**
