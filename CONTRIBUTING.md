# The Concepts
The code is organized around three core concepts: Tricks, Actions, and Providers.

Tricks are the core of this program. A trick can be either a noun or a verb. Usually, a trick is some package that can be installed or run. But sometimes, a trick is just a package (Decky) that can't be run directly, or an action ("clean up disk space").

Actions are what can be done with each trick. Generally, it's something like: `run`, `install`, `uninstall`, etc.

Providers are what handle actions for tricks. Think "the thing that actually talks to `flatpak`".

# The Architecture
```
src/ - The core library and CLI code
gui/rust/ - The Rust interaction layer between the GUI and the core library
gui/godot/ - The Godot GUI code
.github/workflows/ - The GitHub Actions code
ci_scripts/ - The scripts used for building/testing in GitHub Actions
build_assets/ - The static installer/runner files (.sh and .desktop)
```
The core library (used in the CLI and 

# Testing
For changes to the core library, run `cargo test`.

To test changes to the GUI, run `cd gui/rust/ && cargo test --release`.

To run the GitHub Actions flow locally, run `act`.

# Prerequisites
* `pacman -S flatpak fontconfig`
* systemctl/systemd (You probably already have this.)
* Rust (see [https://rustup.rs/](rustup.rs))
* Godot >4.4 (12/24: This requires a dev build for now, see the archives in [misc/act-docker](./misc/act-docker))

The Docker image used by `act` has all the dependencies needed for runs: [gleesus/decktricks](https://hub.docker.com/r/gleesus/decktricks)

The Dockerfile is here: [misc/act-docker/Dockerfile](./misc/act-docker/Dockerfile)
