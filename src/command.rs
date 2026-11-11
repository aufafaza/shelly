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

use anyhow::bail;
use std::fs::{remove_dir, remove_file};

pub enum Command {
    BuiltIn(BuiltIn),
    External {
        program: String,
        args: Vec<String>,
        background: bool,
    },
}

pub enum BuiltIn {
    Cd(String),
    Cat(String),
    Echo(String),
    Pwd,
    Ls,
    Mkdir(String),
    Touch(String),
    Exit,
}

pub enum Execution {
    Continue,
    Exit,
}

impl Command {
    pub fn parse(input: &str) -> anyhow::Result<Option<Self>> {
        let mut parts: Vec<&str> = input.split_whitespace().collect();
        let background = if parts.last() == Some(&"&") {
            parts.pop();
            true
        } else {
            false
        };
        let command = match parts.as_slice() {
            [] => return Ok(None),
            ["exit"] => Command::BuiltIn(BuiltIn::Exit),
            ["exit", ..] => bail!("exit: expected no arguments"),
            ["cat", path] => Command::BuiltIn(BuiltIn::Cat((*path).to_owned())),
            ["cat", ..] => bail!("cat: expected one filename"),
            ["cd", path] => Command::BuiltIn(BuiltIn::Cd((*path).to_owned())),
            ["cd", ..] => bail!("cd: expected a dir"),
            ["echo", str_] => Command::BuiltIn(BuiltIn::Echo((*str_).to_owned())),
            ["pwd"] => Command::BuiltIn(BuiltIn::Pwd),
            ["pwd", ..] => bail!("pwd: expected no arguments"),
            ["ls"] => Command::BuiltIn(BuiltIn::Ls),
            ["ls", ..] => bail!("ls: expected no arguments"),
            ["touch", path] => Command::BuiltIn(BuiltIn::Touch((*path).to_owned())),
            ["touch", ..] => bail!("touch: expected a dir"),
            [program, args @ ..] => Command::External {
                program: (*program).to_owned(),
                args: args.iter().map(|arg| (*arg).to_owned()).collect(),
                background,
            },
        };
        Ok(Some(command))
    }
}

fn execute_legacy(input: &str) {
    let str_parts: Vec<&str> = input.split_whitespace().collect();
    if str_parts.is_empty() {
        return;
    }
    // first string before first whitespace
    match str_parts[0] {
        "rm" => {
            // there must be a better way to do this
            // like we shouldn't really rely on strlen
            let parsed_flag: Vec<&str> = str_parts[1].split("-").collect();
            match str_parts.len() {
                2 => {
                    // delete a file
                    if !parsed_flag.is_empty() {
                        eprintln!("must specify directory to use -r flag");
                        return;
                    }
                    if let Err(e) = remove_file(str_parts[1]) {
                        eprintln!("error in removing file: {}", e);
                        return;
                    }
                    println!("successfully removed file: {}", str_parts[1]);
                }
                3.. => {
                    // let parsed_flag: Vec<&str> = str_parts[1].split("-").collect();
                    println!("flag: {}, removing {}", parsed_flag[0], str_parts[2]);
                    match parsed_flag[1] {
                        "r" => {
                            if let Err(e) = remove_dir(str_parts[2]) {
                                eprintln!("error in removing directory: {}", e);
                                return;
                            }
                            println!("successfully removed directory: {}", str_parts[2]);
                        }
                        _ => {
                            eprintln!("flag unimplemented");
                        }
                    }
                }
                _ => {
                    // < 2, error
                    eprintln!("rm: (rm) (filename.[suffix] | dirname)");
                }
            }
        }
        _ => {
            eprintln!("command {} not found", str_parts[0]);
        }
    }
}
