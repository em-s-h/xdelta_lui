# Xdelta - LUI

Xdelta - Linux User Interface <br> 
A Graphical User Interface for patching ROMs/creating patches with xdelta3.

## Demo

https://github.com/em-s-h/xdelta_lui/assets/120836609/84940751-209f-43d0-988e-7d2e1ac3b68a

## Installation

<!-- {{{ -->

### Dependencies

<!-- {{{ -->

To run Xdelta3 - LUI you'll need to install the following dependencies:

- [xdelta3](https://github.com/jmacd/xdelta)
- [gtk4](https://www.gtk.org/docs/installations/)

On Ubuntu:
```shell
sudo apt install xdelta3 libgtk-4-1
```

On Arch linux:
```shell
sudo pacman -S xdelta3 gtk4
```

<!-- }}} -->

---

After installing the dependencies:

1. Download the latest binary from the [releases](https://github.com/em-s-h/xdelta_lui/releases) page;
    - If your gtk4 version is bellow 4.10 you'll have to download the binary from this version
    [v1.0.1](https://github.com/em-s-h/xdelta_lui/releases/tag/v1.0.1).

2. Make the binary executable by either running `chmod 555 path/to/binary` in a terminal, or by
altering its properties in a file explorer.

<!-- }}} -->

## Building from source

<!-- {{{ -->

To build from source: 

1. Install the dependencies above + [cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html);

2. Clone the repository with `git clone https://github.com/em-s-h/xdelta_lui.git`;

3. Run `cargo b -r` inside the repository to build a release binary at `target/release/xdelta_lui`

<!-- }}} -->
