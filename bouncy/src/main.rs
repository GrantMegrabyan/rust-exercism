mod ball;
mod game;
mod parse_args;
mod types;

use parse_args::parse_args;

fn main() {
    match parse_args() {
        Err(err) => eprintln!("{:?}", err),
        Ok(frame) => {
            let mut game = game::Game::new(frame);
            let sleep_duration = std::time::Duration::from_millis(33);
            loop {
                // Clear screen
                print!("\x1B[2J\x1B[1;1H");
                println!("{}", game);
                game.step();
                std::thread::sleep(sleep_duration);
            }
        }
    }
}
