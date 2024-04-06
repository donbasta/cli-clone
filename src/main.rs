use std::io::{self};
use std::path::Path;
use std::fs::{self};
use std::env;

use std::collections::HashMap;
use std::io::Write;

use colored::*;

const MANUAL: &str = "echo: repeats input
cat: concatenate files
ls: list directories
find: locate files or directories
grep: matches text in files
";

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let mut current_dir_path = Path::new(&current_dir);

    println!(r#"
    _____ _      _____   _                 _             _               _        
    / ____| |    |_   _| | |               | |           | |             | |       
   | |    | |      | |   | |__  _   _    __| | ___  _ __ | |__   __ _ ___| |_ __ _ 
   | |    | |      | |   | '_ \| | | |  / _` |/ _ \| '_ \| '_ \ / _` / __| __/ _` |
   | |____| |____ _| |_  | |_) | |_| | | (_| | (_) | | | | |_) | (_| \__ \ || (_| |
    \_____|______|_____| |_.__/ \__, |  \__,_|\___/|_| |_|_.__/ \__,_|___/\__\__,_|
                                 __/ |                                             
                                |___/                                             
    "#);
    println!("Made with ♥ using Rust");
    println!("Type man for list of commands");

    let manual_detail: HashMap<&str, &str> = [
        ("echo", "for what"),
        ("pwd", "for what"),
        ("cd", "for what"),
        ("ls", "for what"),
        ("find", "for what"),
        ("grep", "for what"),
        ("cat", "for what"),
        ("exit", "for what"),
        ("quit","for what"),
        ("man","for what"),
    ].iter().cloned().collect();

    const COMMANDS: [&str; 10] = [
        "echo",
        "pwd",
        "cd",
        "ls",
        "find",
        "grep",
        "cat",
        "exit",
        "quit",
        "man"
    ];

    loop {
        print!("{}", format!(" {}$ ", current_dir_path.display()).white().bold().on_green());
        print!("  ");
        io::stdout().flush().unwrap();

        let mut command = String::new();

        io::stdin().read_line(&mut command).expect("Failed to read line");
        command = command.trim_start_matches(|c| c == ' ').to_string();

        let tokens: Vec<&str> = command.split_whitespace().collect();
        let chars: Vec<char> = command.chars().collect();

        if tokens.len() == 0 {
            continue;
        }

        match tokens[0] {
            "echo" => {
                if tokens.len() >= 2 {
                    let mut itr = 4;
                    while itr < chars.len() && chars[itr] == ' ' {
                        itr += 1;
                    }
                    println!("{}", format!("{}", &command[itr..command.len()].green()));
                }
            },
            "pwd" => {
                print!("{}", format!("{}", current_dir_path.display()).cyan());
            },
            "cd" => {
                if tokens.len() > 2 {
                    println!("{}", format!("{}", "Too many arguments".red()));
                } else if tokens.len() == 2 {
                    let dest = tokens[1];
                    if dest == ".." {
                        current_dir_path = Path::new(current_dir_path).parent().unwrap();
                    } else if dest.starts_with('/') { // absolute path
                        match fs::symlink_metadata(dest) {
                            Ok(metadata) => {
                                if metadata.is_file() {
                                    println!("File exists, but not a path. Can't change directory");
                                } else if metadata.is_dir() {
                                    println!("Directory exists");
                                    current_dir_path = Path::new(dest.to_string().leak());
                                } else {
                                    println!("Not a file nor a directory");
                                }
                            },
                            Err(err) => eprintln!("Error: {}", err)
                        }
                    } else if dest.starts_with("./") { // relative path
                        let absolute_path = current_dir_path.join(Path::new(dest.to_string().leak()));
                        println!("{}", absolute_path.display());
                        match fs::symlink_metadata(absolute_path.clone()) {
                            Ok(metadata) => {
                                if metadata.is_file() {
                                    println!("File exists, but not a path. Can't change directory");
                                } else if metadata.is_dir() {
                                    println!("Directory exists");
                                    current_dir_path = Path::new(absolute_path.to_string_lossy().into_owned().leak());
                                } else {
                                    println!("Not a file nor a directory");
                                }
                            },
                            Err(err) => eprintln!("Error: {}", err)
                        }
                    } else {
                        println!("{}", format!("{}", "No such file or directory".red()));
                    }
                }
            },
            "ls" => {
                if tokens.len() > 2 {
                    print!("{}", "too many arguments".red());
                }

                let dir_path = if tokens.len() > 1 { current_dir_path.join(Path::new(tokens[1])) } else { current_dir_path.to_path_buf() };
                
                if let Ok(entries) = fs::read_dir(dir_path) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let file_name = entry.file_name();
                            println!("{}", file_name.to_string_lossy());
                        }
                    }
                } else {
                    println!("{}", format!("{}", "Failed to read directory contents".red()));
                }
            },
            "cat" => {
                for i in 1..tokens.len() {
                    let fpath = current_dir_path.join(Path::new(tokens[i]));
                    let data = fs::read_to_string(fpath).expect("Unable to read file data");
                    println!("{}", data);
                }
            },
            "find" => {
                println!("to do doing find");
            },
            "grep" => {
                println!("to do doing grep");
            },
            "exit" | "quit" => {
                println!("Exiting CLI");
                std::process::exit(0);
            },
            "man" => {
                if tokens.len() == 1 {
                    println!("{}", "For more detailed manual for each command, type 'man <command name>'");
                    println!("");
                    println!("{}", MANUAL);
                } else {
                    for i in 1..tokens.len() {
                        if COMMANDS.contains(&tokens[i]) {
                            println!("{}", manual_detail[tokens[i]]);
                        } else {
                            println!("Command {} not found", tokens[i].red());
                        }
                    }
                }
            }
            &_ => {
                println!("Command {} not found, see 'man' for help", tokens[0].red().bold());
            }
        }
    }
}
