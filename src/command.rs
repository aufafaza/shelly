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
use std::fs::{read, read_dir, remove_file};
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

        "rm" => {
            if str_parts.len() == 2 {
                if let Err(e) = remove_file(str_parts[1]) {
                    eprintln!("error in removing file: {}", e);
                }
                println!("successfully removed file: {}", str_parts[1]);
            } else {
                eprintln!("rm: (rm) (filename.[suffix])");
            }
        }

        "cat" => {
            if str_parts.len() == 2 {
                // read returns bytes
                if let Ok(output_bytes) = read(str_parts[1]) {
                    let output_with_endlines_unseparated: String =
                        String::from_utf8(output_bytes).unwrap();
                    println!("{}", output_with_endlines_unseparated);
                } else {
                    eprintln!("error with reading file");
                }
            } else {
                eprintln!("cat: (cat) (filename.[suffix])");
            }
        }

        _ => {
            eprintln!("command {} not found", str_parts[0]);
        }
    }
}
