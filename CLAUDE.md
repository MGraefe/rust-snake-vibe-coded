# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a terminal-based Snake game written in Rust using the curses library (ncurses/pancurses). The game runs entirely in the terminal with ASCII/Unicode graphics.

## Custom Agent Available

This repository has a specialized `rust-curses-game-dev` agent available. Use this agent for:
- Game development tasks
- Curses/ncurses-specific issues
- Terminal UI rendering problems
- Input handling and game loop implementation
- Architecture decisions for terminal games

Invoke with: **Task tool with `subagent_type: "rust-curses-game-dev"`**

## Development Commands

### Building and Running
```bash
cargo build          # Build the project
cargo run            # Build and run the game
cargo build --release  # Optimized build for release
cargo run --release    # Run the optimized version
```

### Testing and Linting
```bash
cargo test           # Run all tests
cargo test <test_name>  # Run a specific test
cargo clippy         # Run Rust linter
cargo fmt            # Format code
cargo check          # Fast compile check without binary
```

## Architecture Notes

### Game Structure
The game should maintain separation between:
- **Game State**: Snake position, food position, score, direction, game status
- **Game Logic**: Movement, collision detection, growth, win/loss conditions
- **Rendering Layer**: Drawing the snake, food, borders, and UI to the terminal
- **Input Handling**: Processing keyboard input (arrow keys, pause, quit)

### Curses Lifecycle
All curses-based terminal games must follow this pattern:
1. **Initialization**: `initscr()`, configure settings (`noecho()`, `curs_set(0)`, `keypad()`)
2. **Game Loop**: Input → Update → Render → Sleep/Frame control
3. **Cleanup**: Always call `endwin()` before exit (use Drop trait or defer patterns)

### Key Technical Considerations
- Use `pancurses` for cross-platform support or `ncurses` for Unix-only
- Implement non-blocking input with `timeout()` for smooth gameplay
- Handle terminal resize events gracefully
- Use color pairs for visual appeal (`init_pair`, `attron`)
- Test on different terminal sizes and emulators

### Common Pitfalls
- Forgetting `endwin()` leaves the terminal in a broken state
- Blocking input freezes the game - always use `timeout()`
- Not checking terminal dimensions can cause crashes on small terminals
- Color support varies across terminals - check `has_colors()` before use
