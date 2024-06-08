use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Read, Write},
};

pub struct Arguments {
    pub action: String,
    pub data: String,
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

        match action.as_str() {
            "add" => {
                data = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get the todo that is to be added.".to_string()),
                };
            }
            "delete" => {
                data = match args.next() {
                    Some(arg) => arg,
                    None => return Err("Didn't get the todo id that is to be deleted.".to_string()),
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

        Ok(Arguments { action, data, file })
    }
}

pub fn list(arguments: Arguments) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(arguments.file);
    let mut todos = String::new();

    reader.read_to_string(&mut todos)?;

    println!("{}", todos);

    Ok(())
}

pub fn add(arguments: Arguments) -> Result<(), Box<dyn Error>> {
    let todo = arguments.data;
    let file = arguments.file;
    let reader = BufReader::new(file.try_clone()?);
    let mut writer = BufWriter::new(file);

    let mut line_count = 1;
    for _ in reader.lines() {
        line_count += 1;
    }

    write!(writer, "{}. {}", line_count, todo)?;

    println!("Todo: {} is added", todo);

    Ok(())
}
