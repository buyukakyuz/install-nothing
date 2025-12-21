# install-nothing

A terminal application that simulates installing things. It doesn't actually install anything.

[![asciicast](https://asciinema.org/a/757039.svg)](https://asciinema.org/a/757039)

## Installation

### Download binary

Grab the latest binary for your platform from [Releases](https://github.com/buyukakyuz/install-nothing/releases)

```bash
chmod +x install-nothing-*
./install-nothing-linux-x86_64
```

### Build from source

```bash
cargo run --release
```

Press Ctrl+C to stop.

### Pick what to install

By default we install everything. But you can change this behavior.
```bash
# Install specific stages
cargo run --release -- kernel
```

Or pick what not to install.
```bash
# Exclude specific stages from installation
cargo run --release -- --exclude cloud xorg
```

See available stages:
```bash
cargo run --release -- --help
```


## Docker

Build
```bash
docker build -t install-nothing .
```

Run
```bash
docker run -it --rm --init install-nothing
```
## License

MIT License,see LICENSE file for more information
