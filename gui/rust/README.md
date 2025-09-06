## Building
To build the Rust library and export the GUI binary, run:

```bash
cargo run --release --bin gui-tool -- build-and-export
```

To clean Godot caches and rebuild from scratch:

```bash
cargo run --release --bin gui-tool -- --clean build-and-export
```

## Building by hand
To just build the lib, run `cargo build --release` and it will be placed in `target/release/libdecktricks_godot_gui.so`.

To manually export the GUI, copy the .so to `../godot/build/`, then cd to `../godot` and run:

```bash
godot --headless --export-release linux-release
```
