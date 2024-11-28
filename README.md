# decktricks
Tools for making life easier on the Steam Deck.

# The Code
The code is organized around three core concepts: Tricks, Actions, and Providers.

Tricks are the core of this program. A trick can be either a noun or a verb. Usually, a trick is some package that can be installed or run. But sometimes, a trick is just a package (Decky) that can't be run directly, or an action ("clean up disk space").

Actions are what can be done with each trick. Generally, it's something like: `run`, `install`, `uninstall`, etc.

Providers are what handle actions for tricks. Think "the thing that actually talks to `flatpak`".

# Testing

For changes to the core library, run `cargo test`.

To test changes to the GUI, run `./scripts/run_all_gui_e2e_tests.sh`.

To run the GitHub Actions flow locally, run `act`.
