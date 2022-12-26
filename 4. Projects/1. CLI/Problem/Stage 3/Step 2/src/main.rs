use std::rc::Rc;

mod models;

mod db;

mod io_utils;
mod ui;

mod navigator;
use navigator::Navigator;

fn main() {
    let db = Rc::new(db::JiraDatabase::new("./data/db.json".to_owned()));
    let mut navigator = Navigator::new(Rc::clone(&db));

    loop {
        clearscreen::clear().unwrap();

        let page = match navigator.get_current_page() {
            None => break,
            Some(page) => page,
        };

        if let Err(error) = page.draw_page() {
            println!("Error rendering page: {error}\nPress any key to continue...");
            io_utils::wait_for_key_press();
            continue;
        };

        let input = io_utils::get_user_input();

        let action = match page.handle_input(&input.trim()) {
            Err(error) => {
                println!("Failed to handle input: {error}\nPress any key to continue...");
                io_utils::wait_for_key_press();
                continue;
            }
            Ok(action) => action,
        };

        if let Some(action) = action {
            if let Err(error) = navigator.handle_action(action) {
                println!("Error handling action: {error}\nPress any key to continue...");
                io_utils::wait_for_key_press();
            };
        }
    }
}
