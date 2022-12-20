use std::rc::Rc;
use std::sync::Mutex;

use game_manager::Cell;
use game_manager::GameManager;

use gtk::{prelude::*, Box};
use gtk::{Application, ApplicationWindow, Button};
use gtk4 as gtk;

mod game_manager;
fn main() {
    let application = Application::builder().application_id("Bomb.Game").build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Bomb Game")
            .maximized(true)
            .build();

        let mut game = GameManager::new(20, 15, None);

        game.generate();

        let game_mutex = Rc::new(Mutex::new(game.clone()));

        let v_box = Box::new(gtk::Orientation::Vertical, 3);
        v_box.set_homogeneous(true);

        for (index, cells) in game.cells.chunks(game.width + 2).enumerate() {
            let h_box = Box::new(gtk::Orientation::Horizontal, 3);
            h_box.set_homogeneous(true);

            for (i, cell) in cells.iter().enumerate() {
                let button = Button::new();

                if cell.0 == true {
                    button.set_sensitive(false);
                    button.set_opacity(0.3);
                }

                let g = game_mutex.clone();

                button.connect_clicked(move |button| {
                    let mut g = g.lock().unwrap();
                    let cell_index = (index * (game.width + 2)) + i;

                    let cell = g.reveal(cell_index);

                    match cell.1 {
                        Cell::NearBomb(number) => button.set_label(&number.to_string()),
                        Cell::Bomb => button.set_label("B"),
                        _ => button.set_opacity(0.3),
                    }
                });

                h_box.append(&button);
            }

            v_box.append(&h_box);
        }

        window.set_child(Some(&v_box));

        window.show();
    });

    application.run();
}
