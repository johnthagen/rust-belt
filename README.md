# Rust Belt
[![Build Status](https://travis-ci.org/johnthagen/rust-belt.svg)](https://travis-ci.org/johnthagen/rust-belt)

`rust_belt` is a recreation of the classic 
[Asteroids](https://en.wikipedia.org/wiki/Asteroids_(video_game)) arcade game using
the [Rust](https://www.rust-lang.org/en-US/) game engine, [Piston](http://www.piston.rs/).

# Usage

## Windows

1. Install the [GNU ABI](https://www.rust-lang.org/en-US/downloads.html#win-foot) version of 
`rustc`.
2. Install [MSYS2](https://msys2.github.io/).
3. Open an MSYS2 terminal
* `pacman -S mingw-w64-x86_64-gcc`
* Add `C:\msys64\mingw64\bin` to system `PATH`.

## Mac OSX

TODO

## Build and Run

To build:

```bash
$ cargo build
```

To run:

```bash
$ cargo run --release
```

# Maintainers
* [@johnthagen](https://github.com/johnthagen)
* [@theandrewdavis](https://github.com/theandrewdavis)