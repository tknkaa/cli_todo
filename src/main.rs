use cli_todo::{create_todo, delete_todo, read_todo, update_todo, ToDos};
use std::io;

fn main() {
    println!("You can create, read, update or delete your todos.");
    let mut todos: ToDos = vec![];
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input: Vec<String> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let action = match input.get(0) {
            Some(action) => action.clone(),
            None => {
                eprintln!("No input");
                return;
            }
        };

        match action.as_str() {
            "create" => create_todo(&mut todos, &input),
            "read" => read_todo(&mut todos),
            "update" => update_todo(&mut todos, &input),
            "delete" => delete_todo(&mut todos, &input),
            _ => {
                eprintln!("Please type proper action");
            }
        }
    }
}
