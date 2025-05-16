use engine::game::Game;

mod engine;
mod structures;
mod ui;

fn main() {
    let mut game = Game::new();
    game.run();
}
