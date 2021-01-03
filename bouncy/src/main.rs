mod game;
mod types;
mod ball;

fn main() {
    let mut  game = game::Game::new();
    let sleep_duration = std::time::Duration::from_millis(33);
    loop {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", game);
        game.step();
        std::thread::sleep(sleep_duration);
    }
}





