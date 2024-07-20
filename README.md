# Rust-todo-CLI
A simple todo list CLI app build in Rust (for learning purpose).

This project use the following crates:
- [clap](https://crates.io/crates/clap) for parsing command line arguments.
- [serde](https://crates.io/crates/serde) for serializing and deserializing data.
- [serde_json](https://crates.io/crates/serde_json) for JSON serialization.

## How to run the app

1. Clone the repository

```
$ git@github.com:Tourlat/rust-todo-CLI.git
```

2. Run `cargo run` in the project root directory with the following arguments:
    - `add 'todo item'` to add a new todo item. 
    - `list` to list all todo items.
    - `delete {todo id}` to delete a todo item.
    - `complete {todo id}` to mark a todo item as complete.
    - `undone {todo id}` to mark a todo item as undone.

## Examples
 
 ```
 $ cargo run add "Learn rust"
 $ cargo run complete 1
 $ cargo run undone 1
```

---
## Note

This project is a work in progress. I will be adding more features and improvements in the future.
