// we want to at least implement
/*
- echo
- ls
- pwd
- cd
- touch
- rm
- cat
* */

use env::{current_dir, set_current_dir};
pub fn execute_command(input: &str) {
    let str_parts: Vec<&str> = input.split_whitespace().collect();
    if str_parts.is_empty() {
        return;
    }
    // first string before first whitespace
    match str_parts[0] {
        "echo" => println!("{}", str_parts[1..].join(" ")),

        "pwd" => match current_dir() {
            Ok(dir) => println!("you are in: {}", dir.display()),
            Err(e) => println!("error in getting current directory, e: {}", e),
        },

        "cd" => {
            if str_parts.len() == 2 {
                if let Err(e) = set_current_dir(str_parts[1]) {
                    eprintln!("error in changing directory: {}", e);
                }
            } else {
                eprintln!("cd: (cd) (directory)");
            }
        }
        _ => {
            eprintln!("command {} not found", str_parts[0]);
        }
    }
}
