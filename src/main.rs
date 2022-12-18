use game_manager::GameManager;

mod game_manager;

fn main() {
    let mut game = GameManager::new(10, 20, None);

    game.generate();

    game.show();
}
