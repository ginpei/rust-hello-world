# Hello Rust

## Install

- [Install Rust - Rust Programming Language](https://www.rust-lang.org/tools/install)

To install on WSL:

```console
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To uninstall:

```consllle
$ rustup self uninstall
```

## Hello, world!

- [Hello, World! - The Rust Programming Language](https://doc.rust-lang.org/book/ch01-02-hello-world.html)

## I'm feeling lucky

### `<>`, `'a`, and lifetime

- [Lifetimes](https://doc.rust-lang.org/1.6.0/book/lifetimes.html)

> The `'a` reads ‘the lifetime a’. Technically, every reference has some lifetime associated with it, but the compiler lets you elide (i.e. omit, see "Lifetime Elision" below) them in common cases.

### `#[derive(Debug)]`

### `Ok()`

## Errors

### Compile error on WSL

```console
$ rustc main.rs 
error: linker `cc` not found
  |
  = note: No such file or directory (os error 2)

error: aborting due to previous error
```

```console
$ sudo apt install build-essential
```

### Weird type mismatches

```rust
let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err);
    process::exit(1);
});
```

```
mismatched types
expected struct `Config`, found `()`rustc (E0308)
```

Import something.

```rust
use std::process;
```
