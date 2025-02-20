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
        let action = input[0].clone();

        if action == "create" {
            let todo = input[1].clone();
            create_todo(&mut todos, todo);
        } else if action == "read" {
            read_todo(&todos);
        } else if action == "quit" {
            break;
        } else if action == "delete" {
            let todo_to_delete = input[1].clone();
            delete_todo(&mut todos, todo_to_delete);
        } else if action == "update" {
            let todo_to_delete = input[1].clone();
            let todo_to_insert = input[2].clone();
            update_todo(&mut todos, todo_to_delete, todo_to_insert);
        } else {
            eprintln!("please type proper action");
        }
    }
}

fn create_todo(todos: &mut Vec<String>, todo: String) {
    todos.push(todo);
}

fn read_todo(todos: &Vec<String>) {
    print!("ToDos: ");
    for todo in todos.iter() {
        print!("{} ", todo);
    }
    print!("\n");
}

fn delete_todo(todos: &mut Vec<String>, todo_to_delete: String) {
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

fn update_todo(todos: &mut Vec<String>, todo_to_delete: String, todo_to_insert: String) {
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
