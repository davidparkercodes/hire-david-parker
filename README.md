# Interactive Resume CLI

An interactive terminal-based resume application built with Rust.

## Features

- Interactive TUI (Terminal User Interface) for exploring resume content
- Multiple sections: About, Skills, Projects, Why Warp
- Always-visible menu sidebar with navigation capabilities
- Clean command-line interface with standard commands

## Installation

### From crates.io

```bash
# Install directly from crates.io
cargo install hiredavidparker
```

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd hire-david-parker

# Build the application
cargo build --release

# The binary will be located at ./target/release/hiredavidparker
```

## Usage

### As a CLI Application

Run the application in interactive TUI mode:

```bash
hiredavidparker
# or
hiredavidparker run
```

Show the about information:

```bash
hiredavidparker about
```

### As a Library

Add to your Cargo.toml:

```toml
[dependencies]
hiredavidparker = "0.1.0"
```

Example usage:

```rust
use hiredavidparker::{about, skills, projects};

fn main() {
    // Get content from various sections
    println!("{}", about());
    
    // Load timeline data
    if let Ok(timeline_events) = hiredavidparker::load_timeline_data() {
        for event in timeline_events {
            println!("{}: {} at {}", event.year, event.title, event.organization);
        }
    }
}
```

## Development

This project follows specific development guidelines. Please refer to the UPDATERULES.md file for more information.

### Running Tests

```bash
cargo test
```

### Test Coverage

We use cargo-tarpaulin for test coverage reporting:

```bash
# Install cargo-tarpaulin (if not already installed)
cargo install cargo-tarpaulin

# Run the coverage script
./scripts/coverage.sh

# Or run tarpaulin directly
cargo tarpaulin --verbose --workspace --skip-clean --out Html --output-dir coverage
```

The HTML coverage report will be generated in the `coverage` directory.

## Technologies

- [Rust](https://www.rust-lang.org/)
- [Clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI library
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation library

## License

This project is licensed under the MIT License - see the LICENSE file for details.