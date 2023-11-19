use std::rc::Rc;

mod models;
mod db;
use db::*;
mod ui;
mod io_utils;
use io_utils::*;
mod navigator;
use navigator::*;

fn main() {
    let db = Rc::new(db::JiraDatabase::new("./data/db_dev.json".to_owned()));
    let mut navigator = navigator::Navigator::new(db);

    loop {
        clearscreen::clear().unwrap();
        if let Some(page) = navigator.get_current_page() {
            if let Err(pg_err) = page.draw_page() {
                println!("Error rendering page:{}.\nPress any key to continue...", pg_err);
                wait_for_key_press();
            }
            let user_input = io_utils::get_user_input();
            match page.handle_input(user_input.trim()) {
                Err(err) => {
                    println!("Error handling input: {}\nPress any key to continue...", err);
                    wait_for_key_press();
                }
                Ok(pg_action) => {
                    if let Some(pg_action) = pg_action {
                        if let Err(err) = navigator.handle_action(pg_action) {
                            println!("Error handling action: {}\nPress any key to continue...", err);
                            wait_for_key_press();
                        }
                    }
                }
            }
        } else {
            break;
        }
    }
}
