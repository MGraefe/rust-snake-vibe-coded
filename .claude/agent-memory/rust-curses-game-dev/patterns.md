# Reusable Patterns for Rust Curses Games

## Curses Initialization Template

```rust
let window = initscr();
pancurses::curs_set(0);        // Hide cursor
pancurses::noecho();           // Don't echo input
pancurses::cbreak();           // Disable line buffering
window.keypad(true);           // Enable arrow keys
window.timeout(0);             // Non-blocking input

if pancurses::has_colors() {
    pancurses::start_color();
    pancurses::init_pair(1, pancurses::COLOR_GREEN, pancurses::COLOR_BLACK);
}
```

## Non-Blocking Input Loop

```rust
match window.getch() {
    Some(Input::Character('q')) => { /* quit */ }
    Some(Input::KeyUp) => { /* handle up */ }
    None => { /* no input this frame */ }
    _ => {}
}
```

## Frame Rate Control

```rust
const FRAME_DURATION: Duration = Duration::from_millis(100);

loop {
    handle_input();
    update_game();
    render();
    thread::sleep(FRAME_DURATION);
}
```

## Safe Cleanup with Drop

```rust
struct Renderer {
    window: Window,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        endwin(); // Always called, even on panic
    }
}
```

## Direction Buffering (Anti-180 Turn)

```rust
struct GameState {
    direction: Direction,
    next_direction: Direction,
}

impl GameState {
    fn set_direction(&mut self, new: Direction) {
        if !self.direction.is_opposite(&new) {
            self.next_direction = new;
        }
    }

    fn update(&mut self) {
        self.direction = self.next_direction; // Apply buffered direction
        // ... rest of update
    }
}
```

## Terminal Size Handling

```rust
fn check_terminal_size(&self) -> bool {
    let max_y = self.window.get_max_y();
    let max_x = self.window.get_max_x();
    max_y >= MIN_HEIGHT && max_x >= MIN_WIDTH
}

fn get_game_area(&self) -> (i32, i32) {
    let max_y = self.window.get_max_y();
    let max_x = self.window.get_max_x();

    // Reserve space for UI elements
    let game_height = max_y - UI_HEIGHT_RESERVED;
    let game_width = max_x - UI_WIDTH_RESERVED;

    (game_width, game_height)
}
```

## Color Usage

```rust
// Initialize in setup
pancurses::init_pair(1, pancurses::COLOR_GREEN, pancurses::COLOR_BLACK);

// Use in rendering
let color = pancurses::COLOR_PAIR(1);
window.attron(color);
window.mvaddch(y, x, 'X');
window.attroff(color);
```

## VecDeque for Snake/Trail

```rust
let mut snake = VecDeque::new();

// Add to front (new head)
snake.push_front(new_head);

// Remove from back (tail movement)
snake.pop_back();

// Keep tail when growing (skip pop_back)
if ate_food {
    // Don't pop
} else {
    snake.pop_back();
}
```

## Collision Detection Patterns

```rust
// Wall collision
if pos.x < 0 || pos.x >= width || pos.y < 0 || pos.y >= height {
    return true; // collision
}

// Self collision (snake)
if snake.iter().any(|segment| *segment == new_head) {
    return true; // collision
}

// Item collision
if snake_head == item_pos {
    // collect item
}
```

## Random Position Generation

```rust
fn spawn_item(&mut self) {
    let mut rng = rand::thread_rng();
    loop {
        let pos = Point {
            x: rng.gen_range(0..self.width),
            y: rng.gen_range(0..self.height),
        };

        // Check if position is valid
        if !self.is_occupied(pos) {
            self.item = pos;
            break;
        }
    }
}
```
