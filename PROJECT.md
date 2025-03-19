# Project Commands and Structure

## Build and Test Commands

```bash
# Build the project
cargo build

# Run the tests
cargo test

# Generate test coverage report
cargo tarpaulin --verbose --workspace --skip-clean --out Html --output-dir coverage

# Generate test coverage excluding runner.rs (recommended)
./scripts/coverage.sh
# Or use the simplified alias
./cov.sh
```

## Codebase Structure

The project is a Rust-based terminal UI (TUI) application for displaying an interactive resume.

### Key Modules

- `src/tui/models.rs` - Data structures including DisplayMode, TimelineEvent, SkillsData
- `src/tui/state.rs` - Application state management (App struct) and initialization
- `src/tui/handlers.rs` - Event handling logic for keyboard and mouse events
- `src/tui/runner.rs` - Main application loop and terminal setup/teardown
- `src/tui/markdown.rs` - Markdown parsing functionality
- `src/tui/ui.rs` - UI rendering components

### Code Style Preferences

- Minimal comments, focus on self-documenting code
- Emphasize separation of concerns
- Follow Rust's API Guidelines
- Maintain high test coverage