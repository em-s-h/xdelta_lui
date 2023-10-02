# Xdelta - LUI

Xdelta - Linux User Interface <br>
A Linux User Interface for patching ROMs with xdelta3

## Installation

<!-- {{{ -->

### Dependencies

<!-- {{{ -->

Before intalling Xdelta3 - LUI you'll need to install the following dependencies:

#### [`xdelta3`](https://github.com/jmacd/xdelta)

To install it on Ubuntu and derivatives run the following command in a terminal:
```shell
sudo apt install xdelta3
```

#### [`gtk4`](https://github.com/jmacd/xdelta)

To install it on Ubuntu and derivatives run the following command in a terminal:
```shell
sudo apt install libgtk-4-1
```

Make sure the version you are installing is bellow `4.10`

<!-- }}} -->

---

After installing the dependencies:

1. Download the [latest](https://github.com/em-s-h/xdelta_lui/releases/tag/release/xdelta_lui)
binary file in the releases page;

2. Give execution permission to the binary file by executing `chmod 555 path/to/binary` in a
terminal or altering the properties of the file in a file explorer.

<!-- }}} -->

## Building from source

<!-- {{{ -->

To build from source: 
1. Install the dependencies above;
2. Download the [source code](https://github.com/em-s-h/xdelta_lui/archive/refs/tags/release.tar.gz)
and extract it;

3. Run `cargo b -r` inside the extracted folder to build a release binary at `target/release/xdelta_lui`

<!-- }}} -->

