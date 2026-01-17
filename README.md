# <img src="https://gitea.com/miviodev/quiet/raw/branch/master/quiet_logo.png" width="32"> quiet 
[🇷🇺 Русская версия](README_RU.md) | 🇬🇧 English version

This utility lets you suppress unnecessary logs, but displays errors when they occur:
```bash
quiet gut clone # instead of git clone
```
Output:
```
[ERROR]: No such file or directory (os error 2)
```
In the latest version, I’ve also added error handling not only for shell-level issues (e.g., command not found) but also for errors within commands themselves:
```
quiet git lig # instead of git log
```
Output:
```
[ERROR]: git: 'lig' is not a git command. See 'git --help'.
The most similar commands are:
	log
```
# Build
To build, you’ll need `Cargo`:
```bash
cargo build -r
```
The binary will be located in `./target/release/`
