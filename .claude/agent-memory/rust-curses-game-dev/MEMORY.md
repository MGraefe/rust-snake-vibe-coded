# Rust Curses Game Development - Agent Memory

## Key Patterns

### Game Architecture
- **Separation of concerns**: GameState (data), Renderer (display), input handling (control)
- **Direction buffering**: Use `next_direction` field to prevent 180-degree turns within one frame
- **Drop trait for cleanup**: Implement Drop on Renderer to guarantee `endwin()` is called
- **Centering windows**: Store offset_x and offset_y in GameState for fixed-size game windows centered in terminal
- **Menu systems**: Use blocking input loop for menus, non-blocking for gameplay
- **Initial pause pattern**: Use `waiting_for_start` bool to prevent movement until first arrow key pressed

### Curses Best Practices
- Always call `keypad(true)` on window to enable arrow key input
- Use `timeout(0)` for non-blocking input (critical for game loops)
- Use `timeout(-1)` for blocking input in menus to prevent flickering
- **Menu flickering fix**: Draw menu once, use blocking input, only redraw when necessary (after dialogs)
- Color pairs: Initialize with `start_color()` and `init_pair()`, apply with `COLOR_PAIR(n)`
- Check `has_colors()` before using colors for terminal compatibility

### Snake Game Specifics
- VecDeque for snake body: `push_front()` for head, `pop_back()` for tail movement
- Opposite direction check prevents instant death from reversing
- Food spawn must check for collision with entire snake body
- Game area calculation: Reserve space for UI (info panel + borders)
- Initial pause prevents wall collision at start: snake waits motionless until arrow key pressed
- Display "Press arrow key to start" message during initial pause (takes priority over other messages)

### Common Pitfalls Avoided
- Terminal size check before gameplay prevents crashes
- Non-blocking input with frame rate control for smooth gameplay
- Direction buffering prevents processing multiple keys in one frame
- Restart functionality reuses game dimensions and offsets from existing state
- Size validation shows error and returns to menu instead of crashing
- Menu flickering from tight loop with non-blocking input: Switch to blocking mode for menus

## Technical Details
- Frame duration: 100ms (10 FPS) works well for classic Snake feel
- Field size presets: Small (30x20), Medium (40x30), Large (80x60)
- Required terminal space: field_width + 2 (borders) by field_height + 5 (info panel + borders)
- Centering formula: offset = (terminal_size - required_size) / 2, clamped to 0
- All rendering uses game.offset_x and game.offset_y for proper positioning

See also: `patterns.md` for reusable code snippets
