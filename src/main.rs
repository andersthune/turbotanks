use turbotanks::Game;

const SETTINGS_PATH: &str = "./turbo_settings.toml";

fn main() {
    let mut game = Game::new(SETTINGS_PATH);

    while game.running() {
        game.update();
    }
}
