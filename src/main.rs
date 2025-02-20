use std::io;
fn main() {
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
    let mut delete_index: Vec<usize> = vec![];
    for (index, todo) in todos.iter_mut().enumerate() {
        if *todo == todo_to_delete {
            delete_index.push(index);
        }
    }
    for index in delete_index.iter() {
        todos.remove(*index);
    }
}
