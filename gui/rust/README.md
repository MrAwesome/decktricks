## Building
To build the rust library and GUI together, just run `cargo test --release`.

You can see the steps for the build in `tests/lib.rs`, under `GODOT_BINARY_PATH`.

## Building by hand
To just build the lib, just do `cargo build --release` and it will be placed in `target/release/libdecktricks_godot_gui.so`.

To build the GUI, you will need to copy the .so to `../godot/build/`, then cd to `../godot` and run:

```bash
godot --headless --export-release Linux
```
