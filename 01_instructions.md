# Instructions

These instructions should be fairly platform-agnostic and straightforward to
follow, but just in case were tested to work on:

 * Windows 10
 * Debian

If you have a different operating system and are having trouble getting
something to work, please contact the course staff.

## Step 1: Installing Rust

To get the most up-to-date installation of Rust, it's recommended to install it
using the [rustup.rs](https://rustup.rs/) website. Linux distributions may
include Rust in their package managers, but unless you use a bleeding-edge
distro like Arch these tend to be out-of-date versions.

[rustup.rs](https://rustup.rs/) will have the most up-to-date instructions, but
for completeness the current version is replicated here:

#### Unix (either Linux or MacOs)
If you are using Unix, run the following in your
terminal and follow the onscreen instructions:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

We will be using the default options in this course: the "default" profile
which includes a variety of tools, and the "stable" toolchain including the
latest stable version of Rust. By default, this script also modifies your
`~/.bashrc` to add the tools it installs to your `$PATH`, which is handy.

Notably, this installation method also works on the `linux.andrew.cmu.edu`
servers! It uses a fair bit of AFS quota space, but is very convenient if
you're already used to doing coding work on those servers.

#### Windows

First, you should install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
from the microsoft website. This isn't required to use Rust in general, but
does allow you to install certain crates (Rust libraries) that need a C
compiler. You may be using some of these libraries for your project, so it's
better to just install these tools now.

Then, download [rustup-init.exe](https://win.rustup.rs/x86_64) from
[rustup.rs](https://rustup.rs/) and run it. We will be using the default
options in this course: the "default" profile which includes a variety of
tools, and the "stable" toolchain including the latest stable version of Rust.
By default, the installer also modifies your `%PATH%` to include the tools it
installs, which is handy.

## Step 2: Setting up the puzzles

If you already have Git installed, you can download all the puzzles by
cloning this repository with:

```bash
git clone https://github.com/Rust-Stuco/puzzles
```

Otherwise, it's fine to download this repository as a `.zip` file from GitHub
and work in that locally.

### Puzzle Structure

Each puzzle is a separate Cargo project living in a separate directory,
containing one or more source files that you'll need to modify. Here is an
example of such a structure:

```text
puzzles/
│
└ 02_pattern_matching/
    │
    ├ Cargo.toml
    ├ Cargo.lock
    ├ README.md
    └ src/
      │
      └ main.rs
```

Here, the only file you should edit is `main.rs`. Sometimes, there will be more
files, in which case the writeup in `README.md` will specify which ones you
should/shouldn't edit.

### Solving Puzzles

The main way you'll test to see if a puzzle is solved or not is by simply
trying to compile it :). Once you figure out all the compilation errors and how
to fix them, you can check the functionality by running `cargo test`. Then, you
can just upload the appropriate source file to Gradescope.

Trivially you can solve any puzzle by deleting almost everything in the file
except for the `main` function, so there's a Gradescope check to make sure you
didn't do that.

<hr/>

[Back to main](./README.md)
