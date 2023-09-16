mod menu;
mod todo;

use menu::Menu;
use todo::Todo;

fn main() {
    let mut todos: Vec<Todo> = Vec::new();

    loop {
        let action = Menu::new();

        match action {
            Menu::Add => action.add(&mut todos),
            Menu::Update => todo!(),
            Menu::Remove => todo!(),
            Menu::List => action.list(&todos),
            Menu::Exit => break,
        }
    }
}
