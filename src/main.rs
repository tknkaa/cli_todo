use std::io;
fn main() {
    println!("You can create, read, update or delete your todos.");
    let mut todos: Vec<String> = vec![];
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

        if action == "create" {
            create_todo(&mut todos, &input);
        } else if action == "read" {
            read_todo(&todos);
        } else if action == "quit" {
            break;
        } else if action == "delete" {
            delete_todo(&mut todos, &input);
        } else if action == "update" {
            update_todo(&mut todos, &input);
        } else {
            eprintln!("Please type proper action");
        }
    }
}

fn create_todo(todos: &mut Vec<String>, input: &Vec<String>) {
    let new_todo = match input.get(1) {
        Some(new_todo) => new_todo.clone(),
        None => {
            eprintln!("Please type new todo");
            return;
        }
    };
    for todo in todos.iter() {
        if *todo == new_todo {
            eprintln!("It already exists.");
            return;
        }
    }
    todos.push(new_todo);
}

fn read_todo(todos: &Vec<String>) {
    print!("ToDos: {:?}", todos);
    print!("\n");
}

fn delete_todo(todos: &mut Vec<String>, input: &Vec<String>) {
    let todo_to_delete = match input.get(1) {
        Some(todo_to_delete) => todo_to_delete.clone(),
        None => {
            eprintln!("Please type todo to delete");
            return;
        }
    };
    let mut delete_index: Option<usize> = None;
    for (index, todo) in todos.iter().enumerate() {
        if *todo == todo_to_delete {
            delete_index = Some(index);
            break;
        }
    }

    match delete_index {
        Some(index) => {
            todos.remove(index);
        }
        None => println!("No result"),
    }
}

fn update_todo(todos: &mut Vec<String>, input: &Vec<String>) {
    let todo_to_delete = match input.get(1) {
        Some(todo_to_delete) => todo_to_delete.clone(),
        None => {
            eprintln!("Please type todo to delete");
            return;
        }
    };

    let todo_to_insert = match input.get(2) {
        Some(todo_to_insert) => todo_to_insert.clone(),
        None => {
            eprintln!("Please type todo to insert");
            return;
        }
    };

    let mut delete_index: Option<usize> = None;
    for (index, todo) in todos.iter().enumerate() {
        if *todo == todo_to_delete {
            delete_index = Some(index);
            break;
        }
    }

    match delete_index {
        Some(index) => {
            todos[index] = todo_to_insert;
        }
        None => println!("No result"),
    }
}
