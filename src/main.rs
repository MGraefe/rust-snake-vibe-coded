use pancurses::{endwin, initscr, Input, Window};
use rand::Rng;
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

// ============================================================================
// GAME CONSTANTS
// ============================================================================

const FRAME_DURATION: Duration = Duration::from_millis(100); // Game speed (100ms = 10 FPS)

// Playing field size options
#[derive(Debug, Clone, Copy)]
struct FieldSize {
    name: &'static str,
    width: i32,
    height: i32,
}

const FIELD_SIZES: [FieldSize; 4] = [
    FieldSize {
        name: "Tiny",
        width: 20,
        height: 10,
    },
    FieldSize {
        name: "Small",
        width: 30,
        height: 20,
    },
    FieldSize {
        name: "Medium",
        width: 40,
        height: 30,
    },
    FieldSize {
        name: "Large",
        width: 60,
        height: 40,
    },
];

// ============================================================================
// GAME STATE STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Returns true if the other direction is opposite to this one
    fn is_opposite(&self, other: &Direction) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
enum GameStatus {
    Playing,
    Paused,
    GameOver,
}

struct GameState {
    snake: VecDeque<Point>,
    direction: Direction,
    next_direction: Direction, // Buffered direction to prevent double-key issues
    food: Point,
    score: u32,
    status: GameStatus,
    game_width: i32,
    game_height: i32,
    offset_x: i32, // Offset for centering the game window
    offset_y: i32, // Offset for centering the game window
    waiting_for_start: bool, // Initial pause until first arrow key press
}

impl GameState {
    fn new(width: i32, height: i32, offset_x: i32, offset_y: i32) -> Self {
        let mut snake = VecDeque::new();
        // Start snake in the center
        let center_x = width / 2;
        let center_y = height / 2;

        snake.push_back(Point {
            x: center_x,
            y: center_y,
        });
        snake.push_back(Point {
            x: center_x - 1,
            y: center_y,
        });
        snake.push_back(Point {
            x: center_x - 2,
            y: center_y,
        });

        let mut game = GameState {
            snake,
            direction: Direction::Right,
            next_direction: Direction::Right,
            food: Point { x: 0, y: 0 }, // Will be set by spawn_food
            score: 0,
            status: GameStatus::Playing,
            game_width: width,
            game_height: height,
            offset_x,
            offset_y,
            waiting_for_start: true, // Start paused until first arrow key
        };

        game.spawn_food();
        game
    }

    /// Generate random food position that doesn't overlap with the snake
    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let food = Point {
                x: rng.gen_range(0..self.game_width),
                y: rng.gen_range(0..self.game_height),
            };

            // Check if food spawned on snake
            if !self.snake.iter().any(|segment| *segment == food) {
                self.food = food;
                break;
            }
        }
    }

    /// Update the direction if the new direction is valid (not opposite)
    fn set_direction(&mut self, new_direction: Direction) {
        if !self.direction.is_opposite(&new_direction) {
            self.next_direction = new_direction;
        }
    }

    /// Main game logic update - called once per frame
    fn update(&mut self) {
        if self.status != GameStatus::Playing {
            return;
        }

        // Don't move snake until player presses first arrow key
        if self.waiting_for_start {
            return;
        }

        // Update direction (prevents 180-degree turns within one frame)
        self.direction = self.next_direction;

        // Calculate new head position
        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Point {
                x: head.x,
                y: head.y - 1,
            },
            Direction::Down => Point {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => Point {
                x: head.x - 1,
                y: head.y,
            },
            Direction::Right => Point {
                x: head.x + 1,
                y: head.y,
            },
        };

        // Check wall collision
        if new_head.x < 0
            || new_head.x >= self.game_width
            || new_head.y < 0
            || new_head.y >= self.game_height
        {
            self.status = GameStatus::GameOver;
            return;
        }

        // Check self collision
        if self.snake.iter().any(|segment| *segment == new_head) {
            self.status = GameStatus::GameOver;
            return;
        }

        // Move snake
        self.snake.push_front(new_head);

        // Check if food was eaten
        if new_head == self.food {
            self.score += 10;
            self.spawn_food();
            // Don't remove tail - snake grows
        } else {
            // Remove tail - normal movement
            self.snake.pop_back();
        }
    }

    fn toggle_pause(&mut self) {
        self.status = match self.status {
            GameStatus::Playing => GameStatus::Paused,
            GameStatus::Paused => GameStatus::Playing,
            GameStatus::GameOver => GameStatus::GameOver,
        };
    }
}

// ============================================================================
// RENDERING LAYER
// ============================================================================

struct Renderer {
    window: Window,
}

impl Renderer {
    fn new() -> Result<Self, String> {
        // Initialize curses
        let window = initscr();

        // Configure curses settings
        pancurses::curs_set(0); // Hide cursor
        pancurses::noecho(); // Don't echo input
        pancurses::cbreak(); // Disable line buffering
        window.keypad(true); // Enable arrow keys
        window.timeout(0); // Non-blocking input

        // Initialize colors if available
        if pancurses::has_colors() {
            pancurses::start_color();
            pancurses::init_pair(1, pancurses::COLOR_GREEN, pancurses::COLOR_BLACK); // Snake
            pancurses::init_pair(2, pancurses::COLOR_RED, pancurses::COLOR_BLACK); // Food
            pancurses::init_pair(3, pancurses::COLOR_YELLOW, pancurses::COLOR_BLACK); // Border
            pancurses::init_pair(4, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK); // Text
        }

        Ok(Renderer { window })
    }

    fn check_size_fits(&self, size: &FieldSize) -> bool {
        let max_y = self.window.get_max_y();
        let max_x = self.window.get_max_x();

        // Need space for: info panel (3 lines), borders (2 chars vertical, 2 horizontal)
        let required_height = size.height + 5; // +3 for info, +2 for borders
        let required_width = size.width + 2;   // +2 for borders

        max_y >= required_height && max_x >= required_width
    }

    fn calculate_offsets(&self, width: i32, height: i32) -> (i32, i32) {
        let max_y = self.window.get_max_y();
        let max_x = self.window.get_max_x();

        // Info panel takes 4 lines (3 lines + 1 blank)
        let total_height = height + 5; // +3 for info, +2 for borders
        let total_width = width + 2;   // +2 for borders

        let offset_y = ((max_y - total_height) / 2).max(0);
        let offset_x = ((max_x - total_width) / 2).max(0);

        (offset_x, offset_y)
    }

    fn show_size_menu(&self) -> Option<usize> {
        // Use blocking input for menu (prevents flickering from tight loop)
        self.window.timeout(-1);

        // Helper function to draw the menu (called once per iteration only when needed)
        let draw_menu = || {
            self.window.clear();

            let start_y = 2;
            let start_x = 2;

            // Title
            let color_pair = pancurses::COLOR_PAIR(4);
            self.window.attron(color_pair);
            self.window.mvprintw(start_y, start_x, "=== RUST SNAKE - SELECT FIELD SIZE ===");
            self.window.attroff(color_pair);

            // Options
            for (i, size) in FIELD_SIZES.iter().enumerate() {
                let y = start_y + 2 + (i as i32 * 2);
                let option_text = format!(
                    "  {}. {} ({}x{})",
                    i + 1,
                    size.name,
                    size.width,
                    size.height
                );

                // Check if this size fits
                if self.check_size_fits(size) {
                    self.window.attron(pancurses::COLOR_PAIR(1));
                    self.window.mvprintw(y, start_x, &option_text);
                    self.window.attroff(pancurses::COLOR_PAIR(1));
                } else {
                    self.window.attron(pancurses::COLOR_PAIR(2));
                    self.window.mvprintw(y, start_x, &format!("{} [TOO LARGE]", option_text));
                    self.window.attroff(pancurses::COLOR_PAIR(2));
                }
            }

            // Instructions
            let y = start_y + 2 + (FIELD_SIZES.len() as i32 * 2) + 1;
            self.window.mvprintw(y, start_x, "Press 1-3 to select a size, or Q to quit");

            let terminal_info = format!(
                "Terminal size: {}x{}",
                self.window.get_max_x(),
                self.window.get_max_y()
            );
            self.window.mvprintw(y + 1, start_x, &terminal_info);

            self.window.refresh();
        };

        // Draw menu once before starting input loop
        draw_menu();

        // Input loop - only redraws when necessary (after error dialog)
        loop {
            // Block and wait for user input (no flickering)
            match self.window.getch() {
                Some(Input::Character('1')) => {
                    if self.check_size_fits(&FIELD_SIZES[0]) {
                        self.window.timeout(0); // Restore non-blocking for gameplay
                        return Some(0);
                    } else {
                        self.show_size_error(&FIELD_SIZES[0]);
                        draw_menu(); // Redraw menu after error dialog
                    }
                }
                Some(Input::Character('2')) => {
                    if self.check_size_fits(&FIELD_SIZES[1]) {
                        self.window.timeout(0); // Restore non-blocking for gameplay
                        return Some(1);
                    } else {
                        self.show_size_error(&FIELD_SIZES[1]);
                        draw_menu(); // Redraw menu after error dialog
                    }
                }
                Some(Input::Character('3')) => {
                    if self.check_size_fits(&FIELD_SIZES[2]) {
                        self.window.timeout(0); // Restore non-blocking for gameplay
                        return Some(2);
                    } else {
                        self.show_size_error(&FIELD_SIZES[2]);
                        draw_menu(); // Redraw menu after error dialog
                    }
                }
                Some(Input::Character('q')) | Some(Input::Character('Q')) => {
                    self.window.timeout(0); // Restore non-blocking before exit
                    return None;
                }
                _ => {
                    // Invalid input - don't redraw, just wait for next input
                }
            }
        }
    }

    fn show_size_error(&self, size: &FieldSize) {
        // Error dialog uses blocking input (already set by show_size_menu)
        self.window.clear();

        let color_pair = pancurses::COLOR_PAIR(2);
        self.window.attron(color_pair);

        let required_width = size.width + 2;
        let required_height = size.height + 5;

        self.window.mvprintw(2, 2, "ERROR: Terminal too small for this field size!");
        self.window.attroff(color_pair);

        self.window.mvprintw(4, 2, &format!("Selected: {} ({}x{})", size.name, size.width, size.height));
        self.window.mvprintw(5, 2, &format!("Required: {}x{}", required_width, required_height));
        self.window.mvprintw(6, 2, &format!("Current:  {}x{}", self.window.get_max_x(), self.window.get_max_y()));

        self.window.mvprintw(8, 2, "Please resize your terminal or select a smaller field size.");
        self.window.mvprintw(9, 2, "Press any key to return to the menu...");

        self.window.refresh();
        // Block and wait for any key press (no timeout needed since parent set it)
        self.window.getch();
    }

    fn render(&self, game: &GameState) {
        self.window.clear();

        // Render top info panel
        self.render_info_panel(game);

        // Render game area
        self.render_game_area(game);

        // Render status messages
        self.render_status_messages(game);

        self.window.refresh();
    }

    fn render_info_panel(&self, game: &GameState) {
        let color_pair = pancurses::COLOR_PAIR(4);
        self.window.attron(color_pair);

        let x = game.offset_x + 1;
        let y = game.offset_y;

        self.window.mvprintw(y, x, &format!("=== RUST SNAKE ==="));
        self.window.mvprintw(y + 1, x, &format!("Score: {}  |  Length: {}  |  Speed: {}ms",
            game.score, game.snake.len(), FRAME_DURATION.as_millis()));
        self.window.mvprintw(y + 2, x, &format!("Controls: Arrow Keys=Move  P=Pause  Q=Quit"));

        self.window.attroff(color_pair);
    }

    fn render_game_area(&self, game: &GameState) {
        // Game area starts below info panel (3 lines + 1 blank = 4)
        let render_offset_y = game.offset_y + 4;
        let render_offset_x = game.offset_x + 1;

        // Draw border
        let border_color = pancurses::COLOR_PAIR(3);
        self.window.attron(border_color);

        // Top and bottom borders
        for x in 0..=game.game_width + 1 {
            self.window.mvaddch(render_offset_y - 1, render_offset_x + x, '#');
            self.window.mvaddch(render_offset_y + game.game_height, render_offset_x + x, '#');
        }

        // Left and right borders
        for y in 0..game.game_height {
            self.window.mvaddch(render_offset_y + y, render_offset_x - 1, '#');
            self.window.mvaddch(render_offset_y + y, render_offset_x + game.game_width, '#');
        }

        self.window.attroff(border_color);

        // Draw food
        let food_color = pancurses::COLOR_PAIR(2);
        self.window.attron(food_color);
        self.window.mvaddch(
            render_offset_y + game.food.y,
            render_offset_x + game.food.x,
            '@',
        );
        self.window.attroff(food_color);

        // Draw snake
        let snake_color = pancurses::COLOR_PAIR(1);
        self.window.attron(snake_color);

        for (i, segment) in game.snake.iter().enumerate() {
            let ch = if i == 0 { 'O' } else { 'o' }; // Head vs body
            self.window.mvaddch(
                render_offset_y + segment.y,
                render_offset_x + segment.x,
                ch,
            );
        }

        self.window.attroff(snake_color);
    }

    fn render_status_messages(&self, game: &GameState) {
        // Position below the game area
        let msg_y = game.offset_y + 4 + game.game_height + 1;
        let msg_x = game.offset_x + 1;

        // Show initial start message (takes priority over other states)
        if game.waiting_for_start {
            let color_pair = pancurses::COLOR_PAIR(3);
            self.window.attron(color_pair);
            self.window.mvprintw(msg_y, msg_x, "*** Press arrow key to start ***");
            self.window.attroff(color_pair);
            return;
        }

        match game.status {
            GameStatus::Paused => {
                let color_pair = pancurses::COLOR_PAIR(3);
                self.window.attron(color_pair);
                self.window.mvprintw(msg_y, msg_x, "*** PAUSED - Press P to continue ***");
                self.window.attroff(color_pair);
            }
            GameStatus::GameOver => {
                let color_pair = pancurses::COLOR_PAIR(2);
                self.window.attron(color_pair);
                self.window.mvprintw(
                    msg_y,
                    msg_x,
                    &format!("*** GAME OVER! Final Score: {} - Press Q to quit or R to restart ***", game.score),
                );
                self.window.attroff(color_pair);
            }
            GameStatus::Playing => {}
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        // Always cleanup curses on exit
        endwin();
    }
}

// ============================================================================
// INPUT HANDLING
// ============================================================================

fn handle_input(window: &Window, game: &mut GameState) -> bool {
    match window.getch() {
        Some(Input::Character('q')) | Some(Input::Character('Q')) => {
            return false; // Quit game
        }
        Some(Input::Character('p')) | Some(Input::Character('P')) => {
            // Don't allow pause during initial waiting state
            if !game.waiting_for_start {
                game.toggle_pause();
            }
        }
        Some(Input::Character('r')) | Some(Input::Character('R')) => {
            if game.status == GameStatus::GameOver {
                // Restart game with same dimensions and offsets
                *game = GameState::new(game.game_width, game.game_height, game.offset_x, game.offset_y);
            }
        }
        Some(Input::KeyUp) => {
            game.set_direction(Direction::Up);
            // Start the game when first arrow key is pressed
            game.waiting_for_start = false;
        }
        Some(Input::KeyDown) => {
            game.set_direction(Direction::Down);
            // Start the game when first arrow key is pressed
            game.waiting_for_start = false;
        }
        Some(Input::KeyLeft) => {
            game.set_direction(Direction::Left);
            // Start the game when first arrow key is pressed
            game.waiting_for_start = false;
        }
        Some(Input::KeyRight) => {
            game.set_direction(Direction::Right);
            // Start the game when first arrow key is pressed
            game.waiting_for_start = false;
        }
        _ => {}
    }

    true // Continue game
}

// ============================================================================
// MAIN GAME LOOP
// ============================================================================

fn main() {
    // Initialize renderer (and curses)
    let renderer = match Renderer::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to initialize renderer: {}", e);
            return;
        }
    };

    // Show size selection menu
    let size_index = match renderer.show_size_menu() {
        Some(idx) => idx,
        None => return, // User quit from menu
    };

    let selected_size = &FIELD_SIZES[size_index];

    // Calculate offsets to center the game window
    let (offset_x, offset_y) = renderer.calculate_offsets(selected_size.width, selected_size.height);

    // Initialize game state with selected size
    let mut game = GameState::new(
        selected_size.width,
        selected_size.height,
        offset_x,
        offset_y,
    );

    // Initial render
    renderer.render(&game);

    // Main game loop
    loop {
        // Handle input
        if !handle_input(&renderer.window, &mut game) {
            break; // User quit
        }

        // Update game logic
        game.update();

        // Render current state
        renderer.render(&game);

        // Frame rate control
        thread::sleep(FRAME_DURATION);
    }

    // Cleanup happens automatically via Renderer's Drop trait
}
