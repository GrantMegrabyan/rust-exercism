mod ball;
mod game;
mod types;

extern crate pancurses;

use pancurses::{curs_set, endwin, initscr};

fn main() -> Result<(), String> {
    let window = initscr();
    window.nodelay(true);
    window.timeout(33);
    curs_set(0);
    let (max_y, max_x) = window.get_max_yx();
    let frame = types::Frame {
        width: max_x as u32 - 2,
        height: max_y as u32 - 2,
    };
    let mut game = game::Game::new(frame);
    loop {
        match window.getch() {
            Some(_) => {
                endwin();
                return Ok(());
            }
            None => {
                window.clear();
                window.border('|', '|', '-', '-', '+', '+', '+', '+');
                window.mvaddch(game.ball.y as i32 + 1, game.ball.x as i32 + 1, 'o');
                window.refresh();
                game.step();
            }
        }
    }
}
