# install-nothing

A terminal application that simulates installing things. It doesn't actually install anything.

[![asciicast](https://asciinema.org/a/757039.svg)](https://asciinema.org/a/757039)

## Usage

```bash
cargo build --release
cargo run --release
```

Press Ctrl+C to stop.

### Pick what to install

```bash
# Install specific stages
cargo run --release -- deno

# Install everything (default)
cargo run --release -- --all
```

See available stages:
```bash
cargo run --release -- --help
```

## License

Do whatever you want with it.
