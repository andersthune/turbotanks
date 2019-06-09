//! Turbo Tanks: A 2D skill-based tank combat arena game

use turbotanks::Game;
use turbotanks::MainMenuState;

const SETTINGS_PATH: &str = "turbo_settings.toml";

fn main() {
    let mut game = Game::new(SETTINGS_PATH);
    let mut menu = MainMenuState { game: &mut game };
    menu.run_state()
}
