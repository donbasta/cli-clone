use::std::io;
// use::std::fs::File;
use::std::fs;
use::std::env;

const MANUAL: &str = "echo: repeats input
cat: concatenate files
ls: list directories
find: locate files or directories
grep: matches text in files
";

fn main() {
    println!("Hello, world!");
    let current_dir = env::current_dir().expect("Failed to get current directory");

    loop {
        let mut command = String::new();
        
        io::stdin().read_line(&mut command).expect("Failed to read line");
        command = command.trim_start_matches(|c| c == ' ').to_string();

        let tokens: Vec<&str> = command.split_whitespace().collect();
        let chars: Vec<char> = command.chars().collect();

        if tokens.len() == 0 {
            continue;
        }   

        print!(">> ");
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
            "ls" => {
                if let Ok(entries) = fs::read_dir(&current_dir) {
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
                println!("to do doing ls");
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
