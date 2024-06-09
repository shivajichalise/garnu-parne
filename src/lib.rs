use std::{
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufWriter, Write},
};

pub struct Arguments {
    pub action: String,
    pub data: String,
    pub new_data: String,
    pub file: File,
}

impl Arguments {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Arguments, String> {
        args.next(); // the first arg will be the path of the executable

        let action = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the action to be done".to_string()),
        };

        let mut data = String::new();
        let mut new_data = String::new();

        match action.as_str() {
            "add" => {
                data = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get the todo that is to be added.".to_string()),
                };
            }
            "edit" => {
                data = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get the todo id that is to be updated.".to_string()),
                };

                new_data = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get the new todo.".to_string()),
                };
            }
            "delete" => {
                data = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get the todo id that is to be deleted.".to_string()),
                };
            }
            "done" => {
                data = match args.next() {
                    Some(arg) => arg,
                    None => {
                        return Err(
                            "Didn't get the todo id that is to be marked as done.".to_string()
                        )
                    }
                };
            }
            "undone" => {
                data = match args.next() {
                    Some(arg) => arg,
                    None => {
                        return Err(
                            "Didn't get the todo id that is to be marked as undone.".to_string()
                        )
                    }
                };
            }
            "list" => {}
            _ => {
                return Err(format!(
                    "Unknown action: {}. Only add, list and delete is supported.",
                    action
                ));
            }
        }

        let file = File::options()
            .read(true)
            .create(true)
            .append(true)
            .open("todos.txt")
            .unwrap();

        Ok(Arguments {
            action,
            data,
            new_data,
            file,
        })
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines(file: File) -> io::Result<io::Lines<io::BufReader<File>>> {
    Ok(io::BufReader::new(file).lines())
}

pub fn list(arguments: Arguments) -> Result<(), Box<dyn Error>> {
    let file = arguments.file;

    let mut line_count = 1;
    if let Ok(lines) = read_lines(file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("{}. {}", line_count, line);
            line_count += 1;
        }
    }

    Ok(())
}

pub fn add(arguments: Arguments) -> Result<(), Box<dyn Error>> {
    let todo = arguments.data;
    let file = arguments.file;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "{} UNDONE", todo)?;

    println!("Todo: {} is added", todo);

    Ok(())
}

pub fn delete(arguments: Arguments) -> Result<(), Box<dyn Error>> {
    let file = arguments.file;
    let temp_file = File::options()
        .read(true)
        .create(true)
        .write(true)
        .open("todos-temp.txt")
        .unwrap();

    let mut writer = BufWriter::new(temp_file);

    let mut line_count = 1;
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            if line_count != arguments.data.parse::<i32>().unwrap() {
                writeln!(writer, "{}", line)?;
            }
            line_count += 1;
        }
    }

    fs::rename("todos-temp.txt", "todos.txt")?;

    Ok(())
}

pub fn edit(arguments: Arguments) -> Result<(), Box<dyn Error>> {
    let file = arguments.file;
    let temp_file = File::options()
        .read(true)
        .create(true)
        .write(true)
        .open("todos-temp.txt")
        .unwrap();

    let mut writer = BufWriter::new(temp_file);

    let mut line_count = 1;
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            if line_count != arguments.data.parse::<i32>().unwrap() {
                writeln!(writer, "{}", line)?;
            } else {
                writeln!(writer, "{}", arguments.new_data)?;
            }
            line_count += 1;
        }
    }

    fs::rename("todos-temp.txt", "todos.txt")?;

    Ok(())
}

fn mark(
    arguments: Arguments,
    status_from: String,
    status_to: String,
) -> Result<(), Box<dyn Error>> {
    let file = arguments.file;
    let temp_file = File::options()
        .read(true)
        .create(true)
        .write(true)
        .open("todos-temp.txt")
        .unwrap();

    let mut writer = BufWriter::new(temp_file);
    let mut line_count = 1;
    if let Ok(lines) = read_lines(file) {
        for line in lines.flatten() {
            if line_count == arguments.data.parse::<i32>().unwrap() {
                writeln!(writer, "{}", line.replace(&status_from, &status_to))?;
            } else {
                writeln!(writer, "{}", line)?;
            }
            line_count += 1;
        }
    }

    fs::rename("todos-temp.txt", "todos.txt")?;

    Ok(())
}

pub fn mark_as_done(arguments: Arguments) -> Result<(), Box<dyn Error>> {
    mark(arguments, "UNDONE".to_string(), "DONE".to_string())?;

    Ok(())
}

pub fn mark_as_undone(arguments: Arguments) -> Result<(), Box<dyn Error>> {
    mark(arguments, "DONE".to_string(), "UNDONE".to_string())?;

    Ok(())
}
