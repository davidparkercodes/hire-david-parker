# Interactive Resume CLI

An interactive terminal-based resume application built with Rust, showcasing professional experience and skills in an engaging format.

## Features

- Interactive TUI (Terminal User Interface) for exploring resume content
- Multiple sections: About, Skills, Projects, Timeline, Contact
- Always-visible menu sidebar with navigation capabilities
- Clean command-line interface with standard commands
- Smooth transitions between different sections
- Project links and detailed timeline events
- Skills visualization with ratings

## Installation

### From crates.io

```bash
# Install directly from crates.io
cargo install hire-david-parker
```

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd hire-david-parker

# Build the application
cargo build --release

# The binary will be located at ./target/release/hire-david-parker
```

## Usage

### As a CLI Application

Run the application in interactive TUI mode:

```bash
hire-david-parker
# or
hire-david-parker run
```

Show the about information:

```bash
hire-david-parker about
```

### Navigation in TUI Mode

When using the interactive TUI mode, you can navigate with the following keys:

- `Tab` / `Shift+Tab`: Navigate between menu items
- `Enter`: Select a menu item
- `Left` / `Right` / `Up` / `Down`: Navigate within content (especially in Timeline view)
- `Esc` / `q`: Quit the application
- `Home`: Return to the main menu from any section

### As a Library

Add to your Cargo.toml:

```toml
[dependencies]
hire-david-parker = "0.1.0"
```

Example usage:

```rust
use hire_david_parker::{about, skills, projects};

fn main() {
    // Get content from various sections
    println!("{}", about());
    
    // Load timeline data
    if let Ok(timeline_events) = hire_david_parker::load_timeline_data() {
        for event in timeline_events {
            println!("{}: {} at {}", event.year, event.title, event.organization);
        }
    }
    
    // Load project links
    if let Ok(project_links) = hire_david_parker::load_project_links() {
        for link in project_links.links {
            println!("{}: {}", link.text, link.url);
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

- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [Ratatui](https://github.com/ratatui-org/ratatui) - Terminal UI library
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation library
- [Serde](https://serde.rs/) - Serialization/deserialization framework

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.