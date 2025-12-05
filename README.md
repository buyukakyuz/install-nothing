# install-nothing

A terminal application that simulates installing things. It doesn't actually install anything.

[![asciicast](https://asciinema.org/a/757039.svg)](https://asciinema.org/a/757039)

## Usage
Clone the repository
```bash
git clone https://github.com/buyukakyuz/install-nothing
```
Go to the repository
```bash
cd install-nothing
```
And run
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

Build and run:

```bash
docker build -t install-nothing .
docker run -it --rm --init install-nothing
```

## License

Do whatever you want with it. Well, except for movies. If you use this in a movie, credit me or something.
