## Quick OS DL

![Preview](/.github/preview.gif)

A terminal-based tool for finding and downloading operating system images. It uses data from the [quickget_cigo](https://github.com/lj3954/quickget_cigo) project to stay up-to-date with new OS releases. Downloads are validated against checksums and the interface is built with Rust using the ratatui library.

## Installation

### Arch Linux

[quickosdl](https://aur.archlinux.org/packages/quickosdl) is available as an [AUR](https://aur.archlinux.org) package.

You can install it using an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers) (e.g. `paru`):

```sh
paru -S quickosdl
```

### Linux and macOS systems

Binary releases can be found on the [releases page](https://github.com/lj3954/quickosdl/releases).
To install, simply download the binary for your system and place it in a directory in your PATH.

Only x86_64 Linux with glibc is currently supported. Linux systems with other libc implementations
or architectures will need to build from source.

### Windows

Windows support is coming soon. Currently, due to a dependency which is currently being rewritten,
a Windows build is not possible.

### Building from source

To build from source, you will need the Rust toolchain, including cargo, installed.

The recommended way to install the Rust toolchain is to use [rustup](https://rustup.rs) provided by the Rust foundation.

Thereafter, you can build the project using `cargo build --release`.
This will produce a binary in the `target/release` directory.
Alternatively, you can install the program directly using `cargo install --path .`.

## Usage

Once the program is installed, you can run it in a terminal using `quickosdl`.
Keybinds are shown within the interface.
