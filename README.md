# Rust Belt

| Build Status |                                                                                |
|--------------|--------------------------------------------------------------------------------|
| Travis       | [![Travis Build Status][travis-build-status-svg]][travis-build-status]         |
| AppVeyor     | [![AppVeyor Build Status][appveyor-build-status-svg]][appveyor-build-status]   |

`rust-belt` is a re-creation of the classic 
[Asteroids](https://en.wikipedia.org/wiki/Asteroids_(video_game)) arcade game using
the [Rust](https://www.rust-lang.org/) game engine, [Piston](http://www.piston.rs/).

#![Rust Belt](./images/rust-belt-logo.jpg)

# Requirements

## Prerequisites

1. The latest stable release of Rust.
    1. Install [`rustup`](https://www.rustup.rs/).

2. `rust-belt` uses [`piston-music`](https://github.com/PistonDevelopers/music) to play music. 
    See its requirements for how to install its [SDL2](https://www.libsdl.org/) dependencies.

## Windows

Select to proceed with either the MSVC or GNU toolchains.

### MSVC (Recommended)

1. Ensure you are using the latest stable 64-bit MVSC toolchain with `rustup show`.
2. Ensure you have installed the 
   [Visual C++ Build Tools](http://landinghub.visualstudio.com/visual-cpp-build-tools)
   (as recommended by `rustup` during install).

### GNU/MinGW-w64

1. Ensure you using the latest stable 64-bit GNU ABI toolchain with `rustup show`.
2. Install [MSYS2](https://msys2.github.io/).
3. In an MSYS2 terminal: `pacman --sync mingw-w64-x86_64-gcc`
4. Add `C:\msys64\mingw64\bin` to system `PATH`.

## Mac OSX

1. Install [Homebrew](http://brew.sh/) (by default this will install `gcc` via Xcode development 
tools).

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

Special thanks to [@aochagavia](https://github.com/aochagavia) for 
[rocket](https://github.com/aochagavia/rocket), which provided many examples to pull from.

## Music

Music composed by [@johnthagen](https://github.com/johnthagen).  All rights reserved.

<!-- Badges -->
[travis-build-status]: https://travis-ci.org/johnthagen/rust-belt
[travis-build-status-svg]: https://travis-ci.org/johnthagen/rust-belt.svg

[appveyor-build-status]: https://ci.appveyor.com/project/johnthagen/rust-belt
[appveyor-build-status-svg]: https://ci.appveyor.com/api/projects/status/nbkgf5i3p4998a2j?svg=true