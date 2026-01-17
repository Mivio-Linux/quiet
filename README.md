# <img src="https://gitea.com/miviodev/quiet/raw/branch/master/quiet_logo.png" width="32"> quiet

🇬🇧 English version | [🇷🇺 Русская версия](./README_RU.md)

[![crates.io](https://img.shields.io/badge/crates-io-yellow.svg)](https://crates.io/crates/quiet-cli)  
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)  
[![Gitea](https://img.shields.io/badge/gitea-logo)](https://gitea.com/miviodev/quiet)  
[![GitHub](https://img.shields.io/badge/github-logo)](https://github.com/miviodev/quiet)

This utility lets you suppress unnecessary output, but displays errors when they occur:

```bash
quiet gut clone # instead of git clone
```

Output:
```
[ERROR]: No such file or directory (os error 2)
```

In the latest version, I’ve also added error handling not only for shell-level issues (e.g., command not found), but also for errors within commands themselves:

```bash
quiet git lig # instead of git log
```

Output:
```
[ERROR]: git: 'lig' is not a git command. See 'git --help'.
The most similar commands are:
	log
```

# Installation

To install, you’ll need `cargo`:

```bash
cargo install quiet-cli
```

If you get `Command 'quiet' not found`, you need to add `~/.cargo/bin` to your PATH:

```bash
echo "export PATH=\$HOME/.cargo/bin:\$PATH" >> ~/.bashrc  # or ~/.zshrc, depending on your shell
```

# Building from source

You’ll also need `cargo` to build from source:

```bash
cargo build -r
```

The binary will be located in `./target/release/`.