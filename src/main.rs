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
            let task = input[1].clone();
            create_todo(&mut todos, task);
        } else if action == "read" {
            read_todo(&todos);
        } else if action == "quit" {
            break;
        } else {
            eprintln!("please type proper action");
        }
    }
}

fn create_todo(todos: &mut Vec<String>, task: String) {
    todos.push(task);
}

fn read_todo(todos: &Vec<String>) {
    print!("ToDos: ");
    for todo in todos.iter() {
        print!("{}, ", todo);
    }
    print!("\n");
}
