use candid::{CandidType, Deserialize};

static mut todos: Vec<ToDo> = Vec::new();
// ToDo
#[derive(CandidType, Deserialize)]
struct ToDo {
    description: String,
    completed: bool,
}

#[ic_cdk::update]
fn add_todo(description: String) -> String {
    unsafe {
        let todo = ToDo::new(description.clone());
        todos.push(todo);
        description
    }
}

#[ic_cdk::update]
fn delete_todo(id: usize) {
    unsafe {
        todos.remove(id as usize);
    }
}

#[ic_cdk::query]
fn get_todos() -> String {
    unsafe {
        let mut result = String::new();
        for todo in &todos {
            result.push_str(&todo.show());
            println!(" ");
        }
        result
    }
}

#[ic_cdk::update]
fn complete_todo(id: usize) {
    unsafe {
        todos[id as usize].completed = true;
    }
}

impl ToDo {
    fn new(description: String) -> ToDo {
        ToDo {
            description: description,
            completed: (false),
        }
    }
    fn show(&self) -> String {
        if self.completed {
            return format!("{} is completed ", self.description).to_string();
        } else {
            return format!("{} is not completed ", self.description).to_string();
        }
    }
}
