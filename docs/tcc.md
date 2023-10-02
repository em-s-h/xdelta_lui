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
I used before had a Linux version some didn't as was the case for a Graphical User Interface (GUI)
for a command line tool I frequently used, instead a different user interface called [YADP](https://github.com/Nhoya/YADP)
was available, but it had many errors and lacked many features.

This forced me to use the command line tool on the terminal, which was very difficult to me. Thus
I have decided to make a GUI for xdelta3 for Linux systems, utilizing the Rust programming language
and the multi-platform GUI toolkit GTK4, this will allow anyone switching from a different OS to
Linux to be able to use the tool without the terminal.

### Theme

A Graphical User Interface (GUI) for the xdelta command line tool.

### Objective

The main objective is to make a Graphical User Interface (GUI) that offers the main features from the
command line tool xdelta3 for begginer Linux users.

### Justification

This theme was chosen because there isn't a linux Graphical User Interface (GUI) for this command
line tool, that has it's main features.

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

| Task                                  | Duration          | Start    | End       |
| ------------------------------------- | :---------------: | :------: | :-------: |
| [TCC](#TCC)                           | 37 days & 5 hours | 04/06/23 | ~16/10/23 |
| [Development](#development)           | 246 days          | 04/06/23 | 01/11/23  |



### TCC

---

| Task                                                         | Duration | Start     | End       |
| ------------------------------------------------------------ | :------: | :-------: | :-------: |
| Chronogram                                                   | 8 days   | 04/06/23  | 11/06/23  |
| Intro + theme + objective + justification + market research  | 2 hours  | 08/06/23  | 08/06/23  |
| Member roles + necessary knowledge                           | 1 hour   | 08/06/23  | 08/06/23  |
| Methodology + tools used                                     | 2 hours  | 01/09/23  | 01/09/23  |
| Logo                                                         | 2 days   | 17/06/23  | 18/06/23  |
| Prototype screen layout                                      | 5 days   | 23/06/23  | 28/06/23  |
| Planning                                                     | 5 days   | 29/08/23  | 03/09/23  |
| Results obtained                                             | 5 days   | ~08/10/23 | ~13/10/23 |
| Conclusion                                                   | 6 days   | ~10/10/23 | ~16/10/23 |



### Development

---

| Task                  | Duration | Start     | End      |
| --------------------- | :------: | :-------: | :------: |
| Study Rust + Rust-gtk | 81 days  | 04/06/23  | 24/08/23 |
| Backend               | 60 days  | 29/08/23  | 31/10/23 |
| Frontend              | 60 days  | 24/08/23  | 31/10/23 |
| Testing               | 45 days  | 01/09/23  | 31/10/23 |



<!-- }}} -->

## Methodology              <!-- {{{ -->

After I switched my operating system from Windows to Linux I realized that there was no Graphical
User Interface (GUI) for the command line tool xdelta3, this forced me to learn how to use the tool in the
command line, which was a very difficult task for a begginer in Linux.

Now after learning how xdelta3 works my goal is to make a GUI for the tool to facilitate the
transition of other people from an other OS to Linux.

Firstly I started to study the Rust programming language and the GTK library, after that I made the
prototype of how the application should look like and then translated this prototype to a xml
template that I could use to initialize the GUI.

Before starting to program the communication between the GUI and xdelta3 I made an activity diagram
of the application, this allowed me to reduce the ammount of refactoring I would have to perform.

### Tools used               <!-- {{{ -->

#### [Neovim](https://neovim.io/)

![neovim](./assets/nvim.svg)

Neovim is a fork of the [Vim](https://www.vim.org/) terminal text and source code editor, engineered
for extensibility and usability, to encourage new applications and contributions. Just like vim,
neovim allows the user to add or create plugins to their setup.

---

#### [Git](https://git-scm.com/)

![git](./assets/git.svg)

Git is a distributed version control system that tracks changes in any set of computer files. Its
goals include speed, data integrity, and support for distributed, non-linear workflows.

---

#### [Github](https://github.com/)

![github](./assets/github.svg)

GitHub is a platform and cloud-based service for software development and version control using Git,
allowing developers to store and manage their code online.

---

#### [PlantUML](https://plantuml.com/)

![plantuml](./assets/plantuml.svg)

PlantUML is a free and open-source drawing tool that allows users to create diagrams from a simple
and human readable text description.

I used PlantUML for this project because it allowed me to easily create an activity diagram of my
application without having to spend too much time on the appearence.

---

#### [Rust](https://www.rust-lang.org/)

![rust](./assets/rust.svg)

Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type
safety, and concurrency. It enforces memory safety by ensuring that all references point to valid
memory, without the need of a garbage collector.

---

#### [Markdown](https://www.markdownguide.org/)

![markdown](./assets/md.svg)

Markdown is a lightweight markup language for creating formatted text using a plain-text editor. It
is widely used in online forums, collaborative software, documentation pages, and readme files.

---

#### [XML](https://www.w3.org/XML/)

![xml](./assets/xml.svg)

XML (Extensible Markup Language) is a markup language and file format for storing, transmitting, and
reconstructing arbitrary data. Its goals are to emphasize simplicity, generality, and usability
across the Internet.

---

#### [GTK](https://gtk.org/)

![gtk](./assets/gtk.svg)

GTK is a free and open-source, cross-platform, object-oriented widget toolkit for creating Graphical
User Interfaces (GUIs). It's written in C with GObject, to enable support for the object-oriented
capabilities.

For this project I used the gtk4-rs package, since it provides safe bindings to the Rust programming
language for the GTK4 library.

---

#### Libadwaita

![libadwaita](./assets/libadwaita.svg)

Libadwaita is an project that serves to extend GTK's base widgets with those specifically conforming
to the GNOME Human Interface Guidelines (HIG).

<!-- }}} -->

<!-- }}} -->
