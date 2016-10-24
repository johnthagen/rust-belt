# Rust Belt
[![Build Status](https://travis-ci.org/johnthagen/rust-belt.svg)](https://travis-ci.org/johnthagen/rust-belt)

`rust-belt` is a re-creation of the classic 
[Asteroids](https://en.wikipedia.org/wiki/Asteroids_(video_game)) arcade game using
the [Rust](https://www.rust-lang.org/en-US/) game engine, [Piston](http://www.piston.rs/).

#![Rust Belt](./images/rust-belt-logo.jpg)

# Requirements

`rust-belt` targets the latest stable release of Rust.

## Music - [SDL2](https://www.libsdl.org/)

`rust-belt` uses [`piston-music`](https://github.com/PistonDevelopers/music) to play music.  See
 its requirements for how to install its dependencies.

## Windows

1. **Recommended:** Install [`rustup`](https://www.rustup.rs/), which will automatically configure
the latest stable 64-bit [GNU ABI](https://www.rust-lang.org/en-US/downloads.html#win-foot)
version of `rustc`.
    * *Alternative:* Manually install the latest 64-bit
[GNU ABI](https://www.rust-lang.org/en-US/downloads.html#win-foot) version of
[`rustc`](https://www.rust-lang.org/downloads.html).
3. Install [MSYS2](https://msys2.github.io/).
4. In an MSYS2 terminal: `pacman --sync mingw-w64-x86_64-gcc`
5. Add `C:\msys64\mingw64\bin` to system `PATH`.

## Mac OSX

1. Install [Homebrew](http://brew.sh/) (by default this will install `gcc` via Xcode development 
tools).
2. Install [`rustup`](https://www.rustup.rs/) and use the default latest stable compiler.

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

Special thanks to [@aochagavia](https://github.com/aochagavia) for his 
[rocket](https://github.com/aochagavia/rocket) which provided many examples to pull from.

## Music

Music composed by [@johnthagen](https://github.com/johnthagen).  All rights reserved.