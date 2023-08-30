# Xdelta-LUI

Xdelta - Linux User Interface

## Members                  <!-- {{{ -->

Emilly Magalhães de Souza:

- Programmer and UI developer.

### Needed knowledge

- Experience with a code editor or IDE. (Vim, VSCode, monodevelop, etc)
- Experience with Git and GitHub.
- Experience with the Rust programming language.
- Experience with Rust-gtk4.

### Obtaining the needed knowledge

- The official Rust documentation.      [^1]
- The official Rust-gtk4 documentation. [^2]
- The official GitHub documentation.    [^3]
- The official Git documentation.       [^4]

[^1]: [Link](https://doc.rust-lang.org/std/index.html)
[^2]: [Link](https://gtk-rs.org/gtk4-rs/stable/latest/docs/gtk4/index.html)
[^3]: [Link](https://docs.github.com/en)
[^4]: [Link](https://git-scm.com/doc)

<!-- }}} -->

## Intro                    <!-- {{{ -->


Near the end of 2022 I had switched my operating system from Windows to Xubuntu, during this time
I was searching for the Linux versions of the apps I had on Windows, while most of the software
I used before had a Linux version some didn't as was the case for an user interface for a coomand
line tool I used, instead a different user interface called [YADP](https://github.com/Nhoya/YADP)
was available, which had many errors and lacked many features.

This forced me to use the command line tool on the terminal, which was very difficult to me. Thus
I have decided to make an xdelta user interface for Linux systems, utilizing the Rust programming
language and the multi-platform GUI toolkit GTK4, this will allow anyone switching from a different
OS to Linux to be able to use xdelta without the terminal.

### Theme

An User Interface for the xdelta command line tool.

### Objective

The main objective is to make an User Interface that offers the main features from the command line
tool xdelta for begginer Linux users.

### Justification

This theme was chosen because there isn't a linux user interface for this command line tool, that
has the main features of the command line tool.

<!-- }}} -->

## Market research          <!-- {{{ -->

### YADP

- Pro: It's a simple bash script.

- Cons: It has bugs and lacks features.

### Rom Patcher JS

- Pros: Supports different patch formats, available in the browser.

- Cons: Loads the entire rom into ram, which can cause issues with a computer with low ram, and it
only applies patches.

<!-- }}} -->

## Chronogram               <!-- {{{ -->

---

| Task                                  | Duration          | Start       | End         |
| ------------------------------------- | ----------------- | ----------- | ----------- |
| [TCC](#TCC)                           | 39 days & 5 hours | 04/06/2023  | ~16/10/2023 |
| [Development](#development)           | 246 days          | 04/06/2023  | 01/11/2023  |



### TCC

---

| Task                                                         | Duration | Start       | End         |
| ------------------------------------------------------------ | -------- | ----------- | ----------- |
| Chronogram                                                   | 8 days   | 04/06/2023  | 11/06/2023  |
| Intro + theme + objective + justification + market research  | 2 hours  | 08/06/2023  | 08/06/2023  |
| Member roles + necessary knowledge                           | 1 hour   | 08/06/2023  | 08/06/2023  |
| Tools used                                                   | 2 hours  | 11/06/2023  | 11/06/2023  |
| Logo                                                         | 2 days   | 17/06/2023  | 18/06/2023  |
| Prototype screen layout                                      | 5 days   | 23/06/2023  | 28/06/2023  |
| Planning                                                     | 7 days   | 01/09/2023  | 08/09/2023  |
| Results obtained                                             | 5 days   | 08/09/2023  | 13/09/2023  |
| Conclusion                                                   | 6 days   | ~10/10/2023 | ~16/10/2023 |



### Development

---

| Task                  | Duration | Start       | End        |
| --------------------- | -------- | ----------- | ---------- |
| Study Rust + Rust-gtk | 81 days  | 04/06/2023  | 24/08/2023 |
| Backend               | 60 days  | 29/08/2023  | 31/10/2023 |
| Frontend              | 60 days  | 24/08/2023  | 31/10/2023 |
| Testing               | 45 days  | 01/09/2023  | 31/10/2023 |



<!-- }}} -->

## Methodology              <!-- {{{ -->

1. screen prototype + study rust/-gtk4
2. translate screen prototype to xml + make an activity diagram of the app
3. start coding

### Tools used               <!-- {{{ -->

#### [Neovim](https://neovim.io/)

Neovim is a open-source [Vim](https://www.vim.org/)-based text & code editor engineered for
extensibility and usability, to encourage new applications and contributions.

![neovim](./assets/neovim.png)

---

#### PlantUML

---

#### [Rust](https://www.rust-lang.org/)

Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type
safety, and concurrency. 

![rust](./assets/rust.svg)

---

#### XML

---

#### [Rust-gtk4](https://gtk-rs.org/)

gtk-rs is a project that provides GTK bindings for the Rust language, this project contains
Rust-gtk4, which provides Rust bindings for GTK4

![gtk4](./assets/rust-gtk.png)

---

#### Adwaita

<!-- }}} -->

<!-- }}} -->
