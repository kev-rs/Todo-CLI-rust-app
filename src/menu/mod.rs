use std::{
    fs,
    io::{self, BufReader, Write},
    process::Command,
};

use crate::todo::Todo;
use ActionFile::{Read, Update};
use chrono::{NaiveDate, DateTime};

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

    pub fn list(&self, mut tasks: &mut Vec<Todo>, show_menu: bool) {
        if show_menu {
            let _ = Command::new("clear").status().unwrap();
            self.sub_menu();
        }

        println!("\n\t\t\t___Todos___");
        for (i, task) in tasks.iter().enumerate() {
            println!(
                "
                    \t\t\t\t\t\t\t\t\t\t\t\tCreated at: {}
{}. [{:?}] [{:?}] {}\n     \t\t\t\t\t\t\t\t\t\t\t\t\t\tUpdated at: {}",
                task.get_created_at(),
                i + 1,
                task.get_status(),
                task.get_priority(),
                task.get_value(),
                task.get_updated_at().unwrap_or(&"".to_string()),
            );
            println!("_____________________________________________________________________________________________________________________________________________________________________________\n");
        }

        if show_menu {
            let menu = Menu::new(false);
            menu.check_actions(&menu, &mut tasks);
        }
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
                let mut tasks = tasks
                    .clone()
                    .into_iter()
                    .filter(|t| t.get_value() != task.get_value())
                    .collect::<Vec<Todo>>();

                self.sync(&mut tasks, Update);
            }
        }
    }

    pub fn edit(&self, tasks: Vec<Todo>) {
        let mut cloned_tasks: Vec<Todo> = tasks.iter().cloned().collect();
        self.list(&mut cloned_tasks, false);

        let selected_task = self.cmd("Edit task: ");
        let selected_task = selected_task.trim().parse::<usize>().unwrap();

        for (i, task) in tasks.iter().cloned().enumerate() {
            let task_id = i + 1;

            if task_id == selected_task {
                let value = self.cmd("Edit value: ").trim().to_string();
                let status = self.cmd("Update status: ").trim().to_string();
                let priority = self.cmd("Update priority: ").trim().to_string();

                let mut tasks = tasks
                    .clone()
                    .into_iter()
                    .map(|mut t| {
                        if task.todo_id == t.todo_id {
                            t.set_value(value.clone().to_string());
                            t.set_status(&status);
                            t.set_priority(&priority);
                            t.set_updated_at();
                        }

                        t
                    })
                    .collect::<Vec<Todo>>();

                self.sync(&mut tasks, Update);
            }
        }
        io::stdin().read_line(&mut String::new()).unwrap();
    }

    fn cmd(&self, txt: &str) -> String {
        let mut buf = String::new();

        print!("{txt}");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();

        buf
    }

    pub fn sync(&self, tasks: &mut Vec<Todo>, action: ActionFile) -> Vec<Todo> {
        let file = match fs::File::open("db.json") {
            Ok(f) => match action {
                Read => f,
                Update => {
                    self.sort(tasks);
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

    fn sort(&self, tasks: &mut Vec<Todo>) {
        if tasks.len() < 2 {
            return;
        }

        let left_len: usize = tasks.len() / 2;
        let right_len: usize = left_len;

        let mut left_arr: Vec<Todo> = Vec::with_capacity(left_len);
        let mut right_arr: Vec<Todo> = Vec::with_capacity(right_len);

        for (i, task) in tasks.iter().enumerate() {
            let task = task.to_owned();
            if i < left_len {
                left_arr.push(task);
            }
            else {
                right_arr.push(task);
            }
        }

        println!("left: {:#?}\nright: {:#?}", left_arr, right_arr);
        self.sort(&mut left_arr);
        self.sort(&mut right_arr);
        self.merge_sort(tasks, &mut left_arr, &mut right_arr, left_len, right_len);
        println!("*left: {:#?}\n*right: {:#?}", left_arr, right_arr);

    }

    fn merge_sort(&self, arr: &mut Vec<Todo>, left_arr: &mut Vec<Todo>, right_arr: &mut Vec<Todo>, left_len: usize, right_len: usize) {
        let (mut i, mut j, mut k) = (0, 0, 0);
        let default_date = "Thu, 01 Jan 5970 00:00:00 +0000";

        while i < left_len && j < right_len {
            let left = DateTime::parse_from_rfc2822(left_arr[i].get_updated_at().unwrap_or(&default_date.to_owned())).unwrap_or_else(|_| DateTime::parse_from_rfc2822(default_date).unwrap());
            let right = DateTime::parse_from_rfc2822(right_arr[j].get_updated_at().unwrap_or(&default_date.to_owned())).unwrap_or_else(|_| DateTime::parse_from_rfc2822(default_date).unwrap());

            match left.cmp(&right) {
                std::cmp::Ordering::Less => {
                    arr[k] = left_arr[i].clone();
                    i += 1;
                },
                std::cmp::Ordering::Greater => {
                    arr[k] = right_arr[j].clone();
                    j += 1;
                },
                std::cmp::Ordering::Equal => {
                    arr[k] = left_arr[i].clone();
                    i += 1;
                },
            }

            k += 1;
        }

        while i < left_len {
            arr[k] = left_arr[i].clone();
            i += 1;
            k += 1;
        }

        while j < right_len {
            arr[k] = right_arr[j].clone();
            j += 1;
            k += 1;
        }

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
