# Garnu Parne

Garnu Parne is a simple command-line tool written in Rust for having a todo list.

## Features

- **List**: View all todos with their status.
- **Add**: Add a new todo to the list.
- **Delete**: Remove a todo from the list by specifying its line number.
- **Edit**: Modify an existing todo by providing its line number and the updated todo.
- **Mark as Done**: Mark a todo as done.
- **Mark as Undone**: Mark a todo as undone.

## Usage

#### List Todos

```
garnu-parne list
```

#### Add Todo

```
garnu-parne add "todo to add"
```

#### Delete Todo

```
garnu-parne delete <line_number>
```

#### Edit Todo

```
garnu-parne edit <line_number> "edited todo"
```

#### Mark as Done

```
garnu-parne done <line_number>
```

#### Mark as Undone

```
garnu-parne undone <line_number>
```

## Getting Started

To get started with YoHo Todo App, follow these steps:

1. Clone this repository.
2. Build the project using Rust.
3. Run the executable file generated.
4. Start managing your todos!

## Contributing

Contributions are welcome! If you have any ideas for improvements or find any bugs, feel free to open an issue or submit a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

