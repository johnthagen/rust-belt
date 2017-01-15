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
    `piston-music` depends on two third-party non-Rust libraries, [SDL2](https://www.libsdl.org/) and 
    [SDL2_mixer](https://www.libsdl.org/projects/SDL_mixer/). Install instructions are provided below.

## Windows

Select to proceed with either the MSVC or GNU toolchain.

### MSVC (Recommended)

1. Ensure you are using the latest stable 64-bit MVSC toolchain with `rustup show` (`stable-x86_64-pc-windows-msvc`).
2. Ensure you have installed the 
   [Visual C++ Build Tools](http://landinghub.visualstudio.com/visual-cpp-build-tools)
   (as recommended by `rustup` during install).
   (e.g. `C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\bin\cl.exe`).
3. [Download](https://www.libsdl.org/download-2.0.php) the latest SDL2 MSVC development library 
(`SDL2-devel-2.0.x-VC.zip`).
4. Unpack and copy all `.lib` files from `SDL2-devel-2.0.x-VC\SDL2-2.0.x\lib\x64\` into a folder and add that folder
to the `LIB` system environment variable.
5. Copy `SDL2.dll` into the `rust-belt` project folder, next to `Cargo.toml`.
6. [Download](https://www.libsdl.org/projects/SDL_mixer/) the latest SDL2_mixer MSVC development library 
(`SDL2_mixer-devel-2.0.x-VC.zip`).
7. Unpack and copy all `.lib` files from `SDL2_mixer-devel-2.0.x-VC\SDL2_mixer-2.0.x\lib\x64` into the same folder
added in step 4 that was added to the `LIB` system environment variable.
8. Copy `SDL2_mixer.dll` and `smpeg2.dll` into the `rust-belt` project folder, next to `Cargo.toml`.

### GNU/MinGW-w64

1. Ensure you using the latest stable 64-bit GNU ABI toolchain with `rustup show` (`stable-x86_64-pc-windows-gnu`).
2. Install [MSYS2](https://msys2.github.io/).
3. In an MSYS2 terminal: `pacman --sync mingw-w64-x86_64-gcc`
4. Add `C:\msys64\mingw64\bin` to system `PATH`.
5. [Download](https://www.libsdl.org/download-2.0.php) the latest SDL2 MinGW development library 
(`SDL2-devel-2.0.x-mingw.tar.gz`).
6. Unpack and copy all `.lib` files from `SDL2-devel-2.0.x-mingw\SDL2-2.0.x\x86_64-w64-mingw32\lib` into a folder and 
add that folder to the `LIBRARY_PATH` system environment variable.
7. Copy `SDL2.dll` into the `rust-belt` project folder, next to `Cargo.toml`.
8. [Download](https://www.libsdl.org/projects/SDL_mixer/) the latest SDL2_mixer MinGW development library 
(`SDL2_mixer-devel-2.0.x-mingw.tar.gz`).
9. Unpack and copy all `.lib` files from `SDL2_mixer-devel-2.0.x-mingw\SDL2_mixer-2.0.x\x86_64-w64-mingw32\lib\` into
the same folder added in step 6 that was added to the `LIB` system environment variable.
10. Copy `SDL2_mixer.dll` and `smpeg2.dll` into the `rust-belt` project folder, next to `Cargo.toml`.

## Mac OSX

1. Install [Homebrew](http://brew.sh/) (by default this will install `gcc` via Xcode development 
tools).
2. `brew install sdl2`
3. `brew install sdl2_mixer --with-flac --with-fluid-synth --with-libmikmod --with-libmodplug --with-libvorbis 
--with-smpeg2`

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