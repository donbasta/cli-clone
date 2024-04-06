use std::io;
// use::std::fs::File;
use std::path::Path;
use std::fs;
use std::env;

const MANUAL: &str = "echo: repeats input
cat: concatenate files
ls: list directories
find: locate files or directories
grep: matches text in files
";

fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let mut current_dir_path = Path::new(&current_dir);
    // let mut command: String;

    loop {
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
                    println!("{}", &command[itr..command.len()]);
                }
            },
            "pwd" => {
                println!("{}", current_dir_path.display());
            },
            "cd" => {
                if tokens.len() > 2 {
                    println!("Too many arguments");
                } else if tokens.len() == 2 {
                    let dest = tokens[1];
                    if dest == ".." {
                        //go back to parent
                        current_dir_path = Path::new(current_dir_path).parent().unwrap();
                    } else if dest.starts_with('/') { // case 1. dest startswith / (absolute path)
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
                    } else if dest.starts_with("./") {
                        // case 2. dest startswith . (relative path)
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
                    }
                } else {
                    println!("No such file or directory");
                }
            },
            "ls" => {
                if let Ok(entries) = fs::read_dir(&current_dir_path) {
                    println!("Files in current directory:");
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let file_name = entry.file_name();
                            println!("{}", file_name.to_string_lossy());
                        }
                    }
                } else {
                    println!("Failed to read directory contents");
                }
            },
            "cat" => {
                // let mut file = File::open()
                println!("to do doing cat");
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
                println!("{}", MANUAL);
            }
            &_ => todo!(),
        }
    }
}
