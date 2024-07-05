# Banscreen

Banscreen is a Rust application that allows you to take screenshots of a window given its title and save it to a
specified file.

## Features

- Screenshots a window given its title
- Saves the screenshot to a specified file

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
Run the application with the `-w` or `--window_title` flag to specify the window title and the `-o` or `--output_file`
flag to specify the output file:

```bash
cargo run -- -w "window title" -o "output_file.png"
```

## Testing

Run the tests with:

```bash
cargo test
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

When applicable it is best to follow TDD and Hexagonal Architecture.

## License

[MIT](https://choosealicense.com/licenses/mit/)