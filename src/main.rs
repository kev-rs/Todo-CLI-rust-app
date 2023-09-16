mod menu;
mod todo;

use menu::{Menu, ActionFile};
use todo::Todo;

fn main() {
    let tasks: Vec<Todo> = Vec::new();

    loop {
        let action = Menu::new(true);
        let mut tasks = action.sync(&tasks, ActionFile::Read);

        match action {
            Menu::Add => action.add(&mut tasks),
            Menu::Update => todo!(),
            Menu::Remove => action.remove(tasks),
            Menu::List => action.list(&mut tasks, true),
            Menu::Exit => break,
        }
    }
}
