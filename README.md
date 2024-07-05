# Banscreen

[![pipeline status](https://git.lab.me/banshee/banscreen/badges/main/pipeline.svg)](https://git.lab.me/banshee/banscreen/-/commits/main)
[![coverage report](https://git.lab.me/banshee/banscreen/badges/main/coverage.svg)](https://git.lab.me/banshee/banscreen/-/commits/main)
[![Latest Release](https://git.lab.me/banshee/banscreen/-/badges/release.svg)](https://git.lab.me/banshee/banscreen/-/releases)

Banscreen is a Rust application that allows you to take screenshots of a window given its title and save it to a
specified file.

## Features

- Screenshots a window given its title
- Lists all windows

## Requirements

- Rust 1.76.0+
- Cargo 1.76.0+

## Installation

Clone the repository:

```bash
git clone https://github.com/ioannisNoukakis/banscreen.git
```

Navigate to the project directory:

```bash
cd banscreen
```

Build the project:

```bash
cargo build --release
```

## Usage

Usage
Run the application with the `capture` subcommand and the `-w` or `--window_title` flag to specify the window title and the `-o` or `--output_file`
flag to specify the output file:

```bash
cargo run -- capture -w "window title" -o "output_file.png"
```

If you are unsure of the window title, use can use the `list` subcommand to list all the window titles:

```bash
cargo run -- list
```

## Testing

Run the tests with:

```bash
cargo test
```

## Roadmap

- [x] Add support to list all the window titles
- [ ] Redo the X11 bindings in TDD
- [ ] Add support for Windows
- [ ] (Maybe add XCB support)
- [ ] (Maybe add support for Wayland)

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

When applicable it is best to follow TDD and Hexagonal Architecture.

## License

[MIT](https://choosealicense.com/licenses/mit/)