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

use std::env::{current_dir, set_current_dir};
use std::fs::File;
use std::fs::read_dir;
use std::io::Write;
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

        "ls" => match read_dir(current_dir().unwrap_or_default()) {
            Ok(entries) => {
                // entries is ReadDir which is a Result<DirEntry, Error>
                for entry in entries.flatten() {
                    println!("{entry:?}");
                }
            }
            Err(e) => {
                eprintln!("ls error: {}", e);
            }
        },

        "touch" => {
            if str_parts.len() == 2 {
                match File::create_new(str_parts[1]) {
                    Ok(_) => {
                        println!("created new file: {}", str_parts[1])
                    }
                    Err(e) => eprintln!("failed to create file {} with error {}", str_parts[1], e),
                }
            } else {
                eprintln!("touch: (touch) (filename.[suffix])");
            }
        }

        _ => {
            eprintln!("command {} not found", str_parts[0]);
        }
    }
}
