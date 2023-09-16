use std::{
    fs,
    io::{self, BufReader, Write},
    process::Command,
};

use crate::todo::Todo;
use ActionFile::{Read, Update};

#[derive(Debug)]
pub enum Menu {
    Add,
    Update,
    Remove,
    List,
    Exit,
}

pub enum ActionFile {
    Read,
    Update,
}

impl Menu {
    pub fn new(start: bool) -> Menu {
        if start {
            let _ = Command::new("clear").status().unwrap();
            println!("\t\t\t***Todo List***");
            println!("1.Add");
            println!("2.Update");
            println!("3.Remove");
            println!("4.List todos");
            println!("5.Exit");
        }

        let mut option = String::with_capacity(1);
        io::stdin().read_line(&mut option).unwrap();

        match option.trim() {
            "1" => Menu::Add,
            "2" => Menu::Update,
            "3" => Menu::Remove,
            "4" => Menu::List,
            "5" => Menu::Exit,
            _ => Menu::Exit,
        }
    }

    fn sub_menu(&self) {
        println!("1.Add\t2.Update\t3.Remove\t4.List todos\t5.Exit");
    }

    pub fn add(&self, tasks: &mut Vec<Todo>) {
        let _ = Command::new("clear").status().unwrap();
        println!("\n\n\t\t\t***Add todo***");

        let mut task = String::new();
        print!("Todo: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut task).unwrap();

        let task = Todo::new(task.trim().to_string());
        tasks.push(task);
        self.sync(tasks, Update);
    }

    pub fn remove(&self, tasks: Vec<Todo>) {
        let mut cloned_tasks: Vec<Todo> = tasks.iter().cloned().collect();
        self.list(&mut cloned_tasks, false);

        let mut selected_task = String::new();
        print!("Delete task No: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut selected_task).unwrap();
        let selected_task: usize = selected_task.trim().parse().unwrap();

        for (i, task) in tasks.iter().cloned().enumerate() {
            let task_id = i + 1;

            if selected_task == task_id {
                let tasks = tasks
                    .clone()
                    .into_iter()
                    .filter(|t| t.get_value() != task.get_value())
                    .collect::<Vec<Todo>>();

                self.sync(&tasks, Update);
            }
        }
    }

    pub fn list(&self, mut tasks: &mut Vec<Todo>, show_menu: bool) {
        if show_menu {
            let _ = Command::new("clear").status().unwrap();
            self.sub_menu();
        }

        println!("\n\t\t\t___Todos___");
        for (i, task) in tasks.iter().enumerate() {
            println!("{}. [{:?}] {}", i + 1, task.get_status(), task.get_value());
            println!("________________________________________________________________________\n");
        }

        if show_menu {
            let menu = Menu::new(false);
            menu.check_actions(&menu, &mut tasks);
        }
    }

    pub fn sync(&self, tasks: &Vec<Todo>, action: ActionFile) -> Vec<Todo> {
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
        let deserialized_tasks: Vec<Todo> =
            serde_json::from_reader(reader).unwrap_or_else(|_| vec![]);

        return deserialized_tasks;
    }

    pub fn check_actions(&self, action: &Menu, mut tasks: &mut Vec<Todo>) -> Option<bool> {
        match action {
            Menu::Add => {
                action.add(&mut tasks);
                None
            }
            Menu::Update => todo!(),
            Menu::Remove => todo!(),
            Menu::List => {
                action.list(&mut tasks, true);
                None
            }
            Menu::Exit => Some(true),
        }
    }
}
