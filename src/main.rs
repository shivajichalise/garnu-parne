use std::env;
use std::process;

use garnu_parne::Arguments;

fn main() {
    let arguments = Arguments::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {err}");
        process::exit(1);
    });

    if arguments.action == "list" {
        if let Err(e) = garnu_parne::list(arguments) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    } else if arguments.action == "add" {
        if let Err(e) = garnu_parne::add(arguments) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    } else if arguments.action == "delete" {
        if let Err(e) = garnu_parne::delete(arguments) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        }
    }
}
