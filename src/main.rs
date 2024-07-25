use rust_todolist::*;
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
  #[clap(subcommand)]
  command: Option<Action>,

}
#[derive(Subcommand)]
pub enum Action {
    Add { task: String },
    Remove { id: u32 },
    Complete { id: u32 },
    Undone { id: u32 },
    Edit {id: u32, task: String},
    List,
}

fn main() {
    let args = Args::parse();

    match &args.command{
      Some(Action::Add{task}) => {
        let mut task_list = load_todo_list().unwrap();
        add_todo( &mut task_list, task);
        save_todo_list(task_list).unwrap();
        println!("Task has been added");
      },
      Some(Action::Remove{id}) => {
        let mut task_list = load_todo_list().unwrap();
        remove_todo(&mut task_list, *id);
        save_todo_list(task_list).unwrap();
        println!("Task with ID {} has been removed", id);
      },
      Some(Action::Complete{id}) => {
        let mut task_list = load_todo_list().unwrap();
        change_todo_compeleted_value(&mut task_list, *id, true);
        save_todo_list(task_list).unwrap();
        println!("Task with ID {} has been completed", id);
      },
      Some(Action::Undone{id}) => {
        let mut task_list = load_todo_list().unwrap();
        change_todo_compeleted_value(&mut task_list, *id, false);
        save_todo_list(task_list).unwrap();
        println!("Task with ID {} is now undone", id);
      },
      Some(Action::List) => {
        let task_list = load_todo_list().unwrap();
        display_todo_list(&task_list);
      },
      Some(Action::Edit{id, task}) => {
        let mut task_list = load_todo_list().unwrap();
        edit_todo_task(&mut task_list, *id, task);
        save_todo_list(task_list).unwrap();
        println!("Task with ID {} has been updated", id);
      },
      None => {
        display_help();
      },

    }
}