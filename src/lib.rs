pub type ToDos = Vec<String>;

pub fn create_todo(todos: &mut Vec<String>, input: &Vec<String>) {
    let new_todo = match input.get(1) {
        Some(new_todo) => new_todo.clone(),
        None => {
            eprintln!("Please type new todo.");
            return;
        }
    };

    if todos.iter().any(|todo| *todo == new_todo) {
        eprintln!("It already exists.");
        return;
    }

    todos.push(new_todo);
}

pub fn read_todo(todos: &Vec<String>) {
    print!("ToDos: {:?}", todos);
    print!("\n");
}

pub fn delete_todo(todos: &mut Vec<String>, input: &Vec<String>) {
    let todo_to_delete = match input.get(1) {
        Some(todo_to_delete) => todo_to_delete.clone(),
        None => {
            eprintln!("Please type todo to delete.");
            return;
        }
    };

    let delete_index = todos.iter().position(|todo| *todo == todo_to_delete);

    match delete_index {
        Some(index) => {
            todos.remove(index);
        }
        None => println!("No result"),
    }
}

pub fn update_todo(todos: &mut Vec<String>, input: &Vec<String>) {
    let todo_to_delete = match input.get(1) {
        Some(todo_to_delete) => todo_to_delete.clone(),
        None => {
            eprintln!("Please type todo to delete.");
            return;
        }
    };

    let todo_to_insert = match input.get(2) {
        Some(todo_to_insert) => todo_to_insert.clone(),
        None => {
            eprintln!("Please type todo to insert.");
            return;
        }
    };

    let delete_index = todos.iter().position(|todo| *todo == todo_to_delete);

    match delete_index {
        Some(index) => {
            todos[index] = todo_to_insert;
        }
        None => println!("No result"),
    }
}
