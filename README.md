## Quick OS DL

![Preview](/.github/preview.gif)

This is a simple TUI tool to allow the user to quickly find URLs to (and download) images of various operating systems.

This project makes use of the JSON data from the [quickget_cigo](https://github.com/lj3954/quickget_cigo) project,
which allows this tool to work reliably, keeping up with new OS releases, without requiring constant updates.

The OS list begins downloading when the program first launches, which will result in fewer delays, as the user
has to make a selection (architecture) beforehand.

## Usage

Binary releases are available on the [releases page](https://github.com/lj3954/quickosdl/releases).

Alternatively, you can build the project yourself by cloning the repository and running `cargo build --release`.

Currently, only Linux and macOS are supported. Windows support is planned in the near future.
