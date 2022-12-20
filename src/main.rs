use std::io::stdin;

use game_manager::GameManager;

mod game_manager;

fn main() {
    let mut game = GameManager::new(30, 35, None);

    game.generate();

    print!("\x1B[2J\x1B[1;1H");

    game.show();

    loop {
        println!();
        println!("insert cell index to reveal : ");
        let mut s = String::new();
        stdin().read_line(&mut s).expect("cant get input");

        let s: usize = s.trim().parse().expect("cant convert to string");

        game.reveal(s);
        print!("\x1B[2J\x1B[1;1H");
        game.show();
    }
}
