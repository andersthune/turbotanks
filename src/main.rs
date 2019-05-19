use turbotanks::Game;

fn main() {
    let mut game = Game::new();

    while game.running() {
        game.update();
    }
}
