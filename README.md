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

## Errors

On WSL.

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
