use todo_list_rust::*;
use clap::Parser;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
  #[clap(subcommand)]
  command: Option<Action>,

}

fn main() {
    let args = Args::parse();

    match &args.command{
      Some(Action::Add{task}) => {
        let mut task_list = load_todo_list().unwrap();
        add_todo( &mut task_list, task);
        save_todo_list(task_list).unwrap();
      },
      Some(Action::Remove{id}) => {
        let mut task_list = load_todo_list().unwrap();
        remove_todo(&mut task_list, *id);
        save_todo_list(task_list).unwrap();
      },
      Some(Action::Done{id}) => {
        let mut task_list = load_todo_list().unwrap();
        change_todo_compeleted_value(&mut task_list, *id, true);
        save_todo_list(task_list).unwrap();
      },
      Some(Action::Undone{id}) => {
        let mut task_list = load_todo_list().unwrap();
        change_todo_compeleted_value(&mut task_list, *id, false);
        save_todo_list(task_list).unwrap();
      },
      Some(Action::List) => {
        let task_list = load_todo_list().unwrap();
        display_todo_list(&task_list);
      },
      Some(Action::Help) => {
        help();
      },
      _ => {
        help();
      }

    }
}