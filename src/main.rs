//! Turbo Tanks: A 2D skill-based tank combat arena game

use turbotanks::Game;

const SETTINGS_PATH: &str = "./turbo_settings.toml";

fn main() {
    let mut game = Game::new(SETTINGS_PATH);
    game.run()
}
