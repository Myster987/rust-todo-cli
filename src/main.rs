use prettytable::{row, Table};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    task: String,
    done: bool,
}

struct TodoApp {
    todos: Vec<Todo>,
}

impl TodoApp {
    fn print_todos(&self) {
        let mut table = Table::new();

        table.add_row(row!["ID", "TASK", "DONE"]);

        for Todo { id, task, done } in self.todos.iter() {
            table.add_row(row![id, task, done]);
        }

        table.printstd();
    }

    fn add_todo(&mut self) {
        let new_task = input("New todo: ");
        let new_todo = Todo {
            id: self.todos.len() + 1,
            task: new_task,
            done: false,
        };
        self.todos.push(new_todo);
    }

    fn delete_todo(&mut self) {
        self.print_todos();
        let prompt = format!(
            "Select todo to delete (number from 1 to {end}): ",
            end = self.todos.len()
        );
        let index_to_delete = input(&prompt).parse::<usize>();

        match index_to_delete {
            Ok(index) => {
                self.todos.remove(index - 1);
                for i in 0..self.todos.len() {
                    self.todos[i].id = i + 1;
                }
            }
            Err(_) => {
                println!("Number expected");
            }
        }
    }

    fn mark_as_done(&mut self) {
        self.print_todos();
        let prompt = format!(
            "Select todo to mark as done (muber from 1 to {end}): ",
            end = self.todos.len()
        );
        let index_to_update = input(&prompt).parse::<usize>();

        match index_to_update {
            Ok(index) => {
                self.todos[index - 1].done = true;
            }
            Err(_) => {
                println!("Number expected.")
            }
        }
    }

    fn edit_todo(&mut self) {
        self.print_todos();

        let prompt = format!(
            "Select todo to mark as done (muber from 1 to {end}): ",
            end = self.todos.len()
        );
        let index_to_edit = input(&prompt).parse::<usize>();

        match index_to_edit {
            Ok(index) => {
                let new_text = input("Enter new task: ");
                self.todos[index - 1].task = new_text;
            }
            Err(_) => {
                println!("Number expected.");
            }
        };
    }

    fn load_todos(&mut self) {
        self.todos = read_json_file("todos.json");
    }

    fn save_todos(&self) {
        write_json_file("todos.json", &self.todos);
    }

    fn start(&mut self) {
        println!("Welcome to todo app!");

        self.load_todos();
        let mut option = input("Option (print, add, delete, edit, done, save, exit): ");

        loop {
            match option.as_str() {
                "print" => self.print_todos(),
                "add" => self.add_todo(),
                "delete" => self.delete_todo(),
                "edit" => self.edit_todo(),
                "save" => self.save_todos(),
                "done" => self.mark_as_done(),
                "exit" => {
                    self.save_todos();
                    break;
                }
                _ => println!("Bad input."),
            }
            option = input("Option (print, add, delete, edit, done, save, exit): ");
        }
    }
}

fn read_json_file(file_path: &str) -> Vec<Todo> {
    let path = Path::new(file_path);
    let file = fs::File::open(path)
        .expect(format!("could not open file `{file_path}`, maybe it doesn't exist").as_str());
    let todos: Vec<Todo> = serde_json::from_reader(file)
        .expect(format!("Given JSON in file `{file_path}` is not valid").as_str());

    todos
}

fn write_json_file(file_path: &str, data: &Vec<Todo>) {
    let path = Path::new(file_path);
    let to_write = serde_json::to_string_pretty(data).expect("Content is invalid");

    fs::write(path, to_write).expect("Unable to write file");
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {}
    }
    input.trim().to_string()
}

fn main() {
    let mut todo_app = TodoApp { todos: vec![] };
    todo_app.start();
}
