use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{self, OpenOptions};
use std::io::{Result, Read};

#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList {
    pub tasks: Vec<Todo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    id: u32,
    description: String,
    completed: bool,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { tasks: vec![] }
    }

    pub fn add_task(&mut self, task: Todo) {
        self.tasks.push(task);
    }
}

impl Todo {
    pub fn new(id: u32, description: String) -> Todo {
        Todo {
            id,
            description,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

pub fn load_todo_list() -> Result<TodoList> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("todo.json")?;

    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    if file_content.is_empty() {
        return Ok(TodoList { tasks: vec![] });
    }

    let task_list: TodoList = serde_json::from_str(&file_content).unwrap();
    Ok(task_list)
}

pub fn save_todo_list(task_list: TodoList) -> Result<()> {
    let file_content = serde_json::to_string(&task_list).unwrap();
    fs::write("todo.json", file_content)?;
    Ok(())
}

pub fn change_todo_compeleted_value(list: &mut TodoList, id: u32, new_value: bool) {
    if let Some(task) = list.tasks.iter_mut().find(|task: &&mut Todo| task.id == id) {
        task.completed = new_value;
    } else {
        println!("Todo task with ID {} not found", id);
    }
}

pub fn adjust_ids(list: &mut TodoList) {
    for (i, task) in list.tasks.iter_mut().enumerate() {
        task.id = i as u32 + 1;
    }
}

pub fn remove_todo(list: &mut TodoList, id: u32) {
    if let Some(task) = list.tasks.iter().position(|task| task.id == id) {
        list.tasks.remove(task);
        adjust_ids(list);
    } else {
        println!("Todo task with ID {} not found", id);
    }
}
pub fn add_todo(list: &mut TodoList, todo_content: &str) {
    let task = Todo {
        id: list.tasks.len() as u32 + 1,
        description: todo_content.to_string(),
        completed: false,
    };
    list.tasks.push(task);
}

pub fn display_todo_list(list: &TodoList) {
    for task in list.tasks.iter() {
        if task.completed {
            println!("{} - {} - [x]", task.id, task.description);
        } else {
            println!("{} - {} - [ ]", task.id, task.description);
        }
    }
}

pub fn display_help() {
    println!("Usage: todo [add|remove|done|undone|list] [task|id|None]");
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_todo() {
        let mut list = TodoList::new();
        add_todo(&mut list, "Test task");
        assert_eq!(list.tasks.len(), 1);
        assert_eq!(list.tasks[0].description, "Test task");
    }

    #[test]
    fn test_remove_todo() {
        let mut list = TodoList::new();
        add_todo(&mut list, "Test task");
        remove_todo(&mut list, 1);
        assert_eq!(list.tasks.len(), 0);
    }

    #[test]
    fn test_change_todo_compeleted_value() {
        let mut list = TodoList::new();
        add_todo(&mut list, "Test task");
        change_todo_compeleted_value(&mut list, 1, true);
        assert_eq!(list.tasks[0].completed, true);
    }

    #[test]
    fn test_adjust_ids() {
        let mut list = TodoList::new();
        add_todo(&mut list, "Test task 1");
        add_todo(&mut list, "Test task 2");
        add_todo(&mut list, "Test task 3");
        remove_todo(&mut list, 2);
        adjust_ids(&mut list);
        assert_eq!(list.tasks[1].id, 2);
    }
}