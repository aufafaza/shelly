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

// pub enum Command {
//     Exit,
//     Echo(String),
//     Ls,
//     Pwd,
//     Cd(String),
//     Touch(String),
//     Rm(String),
//     Cat(String),
//
//
//}
//

pub fn execute_command(input: &str) {
    let str_parts: Vec<&str> = input.split_whitespace().collect();
    if str_parts.is_empty() {
        return;
    }
    // first string before first whitespace
    match str_parts[0] {
        "echo" => println!("{}", str_parts[1..].join(" ")),

        _ => {
            eprintln!("command {} not found", str_parts[0]);
        }
    }
}
