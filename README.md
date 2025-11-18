# TrustMeBro

A Rust macro that wraps arbitrary tokens in an `unsafe` block. Because sometimes you just need the compiler to trust you.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
trustmebro = "0.1.0"
```

## Usage

```rust
use trustmebro::trustmebro;

let mut x = 5;
let ptr = &mut x as *mut i32;

trustmebro! {
    *ptr = 10;
}

assert_eq!(x, 10);
```

## Why?

This macro is a humorous way to write `unsafe` blocks. Instead of:

```rust
unsafe {
    // dangerous code here
}
```

You can write:

```rust
trustmebro! {
    // dangerous code here
}
```

## Safety

This macro provides no additional safety guarantees. It simply wraps your code in an `unsafe` block. Use responsibly and ensure you understand Rust's safety requirements when working with unsafe code.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
