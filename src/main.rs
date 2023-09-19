mod menu;
mod todo;

use menu::{Menu, ActionFile};
use todo::Todo;

fn main() {
    let mut tasks: Vec<Todo> = Vec::new();

    loop {
        let action = Menu::new(true);
        let mut tasks = action.sync(&mut tasks, ActionFile::Read);

        match action {
            Menu::Add => action.add(&mut tasks),
            Menu::Update => action.edit(tasks),
            Menu::Remove => action.remove(tasks),
            Menu::List => action.list(&mut tasks, true),
            Menu::Exit => break,
        }
    }
}
