# Rust Snake - Terminal Snake Game

A classic Snake game implementation in Rust using the pancurses library for cross-platform terminal graphics.

## Features

- Classic snake gameplay with smooth controls
- Color terminal graphics (with fallback for monochrome terminals)
- Score tracking and snake length display
- Pause/resume functionality
- Game over detection with restart option
- Responsive to terminal size
- Cross-platform support (Linux, macOS, Windows)

## Requirements

- Rust 1.70 or newer
- Terminal with minimum 40x20 size
- ncurses library (Linux/macOS) or PDCurses (Windows)

### Installing ncurses (if needed)

**Ubuntu/Debian:**
```bash
sudo apt-get install libncurses5-dev libncursesw5-dev
```

**macOS:**
```bash
brew install ncurses
```

**Windows:**
PDCurses is automatically handled by pancurses - no additional installation needed.

## Building and Running

### Development Build
```bash
cargo build
cargo run
```

### Optimized Release Build
```bash
cargo build --release
cargo run --release
```

## Controls

- **Arrow Keys**: Move the snake (Up, Down, Left, Right)
- **P**: Pause/Resume game
- **R**: Restart game (only available after game over)
- **Q**: Quit game

## Gameplay

1. Control the snake using arrow keys
2. Eat the food (`@`) to grow and increase your score
3. Avoid hitting the walls (marked with `#`)
4. Avoid running into yourself
5. Try to achieve the highest score possible!

## Game Rules

- Each food eaten gives you 10 points
- The snake grows by one segment for each food eaten
- The game ends if you hit a wall or collide with yourself
- You cannot reverse direction (e.g., can't go left while moving right)

## Customization

You can adjust game settings in `src/main.rs`:

- **FRAME_DURATION**: Change game speed (default: 100ms = 10 FPS)
- **MIN_HEIGHT/MIN_WIDTH**: Adjust minimum terminal size requirements
- **Color schemes**: Modify color pairs in `Renderer::new()`
- **Scoring**: Change score increment in `GameState::update()`

## Architecture

The game follows clean separation of concerns:

- **GameState**: Manages snake position, food, score, and game logic
- **Renderer**: Handles all terminal drawing and visual presentation
- **Input handling**: Processes keyboard input with non-blocking reads
- **Main loop**: Coordinates input → update → render cycle

## Troubleshooting

**Terminal too small error:**
- Resize your terminal to at least 40x20 characters
- On some terminals, you may need to adjust font size

**Colors not showing:**
- Some terminals don't support colors - the game will work in monochrome
- Try a different terminal emulator if colors are important

**Game runs too fast/slow:**
- Adjust `FRAME_DURATION` constant in the code
- Lower values = faster game, higher values = slower game

**Controls not responding:**
- Ensure your terminal emulator properly supports ncurses input
- Try running with `TERM=xterm-256color cargo run`

## License

MIT License - feel free to modify and distribute.

## Contributing

Contributions welcome! Some ideas for enhancements:
- Difficulty levels with increasing speed
- Obstacles and power-ups
- High score persistence
- Different game modes
- Sound effects (terminal beep)
