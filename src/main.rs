use game_manager::GameManager;

mod game_manager;

fn main() {
    let mut game = GameManager::new(10, 25, None);

    game.generate();

    game.show();
}
