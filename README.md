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

### Binaries

Binaries are now available for macOS, Linux, and Windows. They can be found on the [releases page](https://github.com/lj3954/quickosdl/releases).
To install, simply download the archive containing the binary for your system, extract it, and place it in a directory in your PATH.

All x86_64 and AArch64 Linux systems are supported, since the linux binaries are statically linked.
macOS binaries are universal, and will work on both Intel and Apple Silicon Macs.
Currently, only x86_64 Windows has binary releases.

On other platforms, use the instructions below to build from source.

### Building from source

To build from source, you will need the Rust toolchain, including cargo, installed.

The recommended way to install the Rust toolchain is to use [rustup](https://rustup.rs) provided by the Rust foundation.

Thereafter, you can build the project using `cargo build --release`.
This will produce a binary in the `target/release` directory.
Alternatively, you can install the program directly using `cargo install --path .`.

## Usage

Once the program is installed, you can run it in a terminal using `quickosdl`.
Keybinds are shown within the interface.
