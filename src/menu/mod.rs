use std::{
    fs,
    io::{self, BufReader, Read, Write},
    process::Command,
};

use crate::todo::Todo;

#[derive(Debug)]
pub enum Menu {
    Add,
    Update,
    Remove,
    List,
    Exit,
}

enum ActionFile {
    Read,
    Update,
}

impl Menu {
    pub fn new() -> Menu {
        let _ = Command::new("clear").status().unwrap();
        println!("\t\t\t***Todo List***");
        println!("1.Add");
        println!("2.Update");
        println!("3.Remove");
        println!("4.List todos");
        println!("5.Exit");

        let mut option = String::with_capacity(1);
        io::stdin().read_line(&mut option).unwrap();

        match option.chars().next() {
            Some('1') => Menu::Add,
            Some('2') => Menu::Update,
            Some('3') => Menu::Remove,
            Some('4') => Menu::List,
            Some('5') => Menu::Exit,
            _ => Menu::Exit,
        }
    }

    pub fn add(&self, tasks: &mut Vec<Todo>) {
        use ActionFile::Update;

        let _ = Command::new("clear").status().unwrap();

        println!("\n\n\t\t\t***Add todo***");

        let mut task = String::new();
        io::stdin().read_line(&mut task).unwrap();

        let task = Todo::new(task.trim().to_string());
        tasks.push(task);

        self.sync(tasks, Update);
    }

    pub fn list(&self, todos: &Vec<Todo>) {
        use ActionFile::Read;

        let _ = Command::new("clear").status().unwrap();
        let todos = self.sync(todos, Read);

        println!("\t\t\t___Todos___");
        println!("{:#?}", todos);

        io::stdin().read_line(&mut String::new()).unwrap();
    }

    fn sync(&self, tasks: &Vec<Todo>, action: ActionFile) -> Vec<Todo> {
        use ActionFile::{Read, Update};

        let file = match fs::File::open("db.json") {
            Ok(f) => match action {
                Read => f,
                Update => {
                    let mut file = fs::OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open("db.json")
                        .unwrap();

                    let serialized_tasks = serde_json::to_string(&tasks).unwrap();
                    file.write_all(serialized_tasks.as_bytes()).unwrap();
                    f
                }
            },
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => {
                    let serialized_tasks = serde_json::to_string(tasks).unwrap();

                    let mut file = fs::File::create("db.json").unwrap();
                    file.write_all(serialized_tasks.as_bytes()).unwrap();
                    file
                }
                other_err => panic!("{}", other_err),
            },
        };
        let reader = BufReader::new(file);
        let deserialized_tasks: Vec<Todo> = serde_json::from_reader(reader).unwrap_or_else(|_| vec![]);

        return deserialized_tasks;
    }
}
