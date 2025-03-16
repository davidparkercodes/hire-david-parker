# Interactive Resume CLI

An interactive terminal-based resume application built with Rust.

## Features

- Interactive TUI (Terminal User Interface) for exploring resume content
- Multiple sections: About, Skills, Projects, Why Warp
- Always-visible menu sidebar with navigation capabilities
- Clean command-line interface with standard commands

## Installation

```bash
# Clone the repository
git clone <repository-url>
cd hire-david-parker

# Build the application
cargo build --release

# The binary will be located at ./target/release/hiredavidparker
```

## Usage

Run the application in interactive TUI mode:

```bash
./target/release/hiredavidparker
# or
./target/release/hiredavidparker run
```

Show the about information:

```bash
./target/release/hiredavidparker about
```

## Development

This project follows specific development guidelines. Please refer to the UPDATERULES.md file for more information.

### Running Tests

```bash
cargo test
```

## Technologies

- [Rust](https://www.rust-lang.org/)
- [Clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI library
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation library

## License

This project is licensed under the MIT License - see the LICENSE file for details.