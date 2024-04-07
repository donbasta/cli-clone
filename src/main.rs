use std::env;
use std::fs::{self};
use std::io::{self};
use std::path::PathBuf;

use std::collections::HashMap;
use std::io::Write;

use colored::*;

const MANUAL: &str = "echo: repeats input
cat: concatenate files
ls: list directories
find: locate files or directories
grep: matches text in files
";

const COMMANDS: [&str; 10] = [
    "echo", "pwd", "cd", "ls", "find", "grep", "cat", "exit", "quit", "man",
];

// fn style_output(text: &str, _vargs: &[&str]) -> ColoredString {
//     let mut colored_text: ColoredString = text.into();
//     for style in _vargs.iter() {
//         match style {
//             &"white" => colored_text = colored_text.white(),
//             &"black" => colored_text = colored_text.black(),
//             &"red" => colored_text = colored_text.red(),
//             &"green" => colored_text = colored_text.green(),
//             &"magenta" => colored_text = colored_text.magenta(),
//             &"blue" => colored_text = colored_text.blue(),
//             &"bold" => colored_text = colored_text.bold(),
//             &"on_green" => colored_text = colored_text.on_green(),
//             &_ => colored_text = colored_text.black(),
//         }
//     }
//     colored_text
// }

struct Variables {
    raw_command: String,
    tokens: Vec<String>,
    chars: Vec<char>,
    current_dir_path: PathBuf,
}

impl Variables {
    pub fn new() -> Self {
        Self {
            raw_command: String::new(),
            tokens: Vec::new(),
            chars: Vec::new(),
            current_dir_path: env::current_dir().expect("Failed to get current directory"),
        }
    }

    pub fn input_and_preprocess(&mut self) {
        self.raw_command = String::new();
        io::stdin()
            .read_line(&mut self.raw_command)
            .expect("Failed to read line");
        self.raw_command = self
            .raw_command
            .trim_start_matches(|c| c == ' ')
            .to_string();
        self.tokens = self
            .raw_command
            .split_whitespace()
            .collect::<Vec<_>>()
            .iter()
            .map(|&s| s.to_owned())
            .collect();
        self.chars = self.raw_command.chars().collect();
    }

    pub fn empty(&self) -> bool {
        return self.tokens.len() == 0;
    }

    pub fn get_first_token(&self) -> &str {
        return &self.tokens[0];
    }

    pub fn get_tokens_length(&self) -> usize {
        return self.tokens.len();
    }

    pub fn display_header(&self) {
        print!(
            "{}",
            format!(" {}$ ", self.current_dir_path.display())
                .white()
                .bold()
                .on_green()
        );
        print!("  ");
        io::stdout().flush().unwrap();
    }
    pub fn run_echo(&self) {
        if self.get_tokens_length() >= 2 {
            let mut itr = 4;
            while itr < self.chars.len() && self.chars[itr] == ' ' {
                itr += 1;
            }
            let tmp: String = self.chars.iter().collect();
            print!("{}", &tmp[itr..self.chars.len()].green());
        }
    }
    pub fn run_pwd(&self) {
        println!("{}", format!("{}", self.current_dir_path.display()).cyan());
    }
    pub fn get_token(&self, idx: usize) -> &str {
        return &self.tokens[idx];
    }
    pub fn run_man(&self) {
        let manual_detail: HashMap<&str, &str> = [
            ("echo", "for what"),
            ("pwd", "for what"),
            ("cd", "for what"),
            ("ls", "for what"),
            ("find", "for what"),
            ("grep", "for what"),
            ("cat", "for what"),
            ("exit", "for what"),
            ("quit", "for what"),
            ("man", "for what"),
        ]
        .iter()
        .cloned()
        .collect();

        if self.get_tokens_length() == 1 {
            println!(
                "{}",
                "For more detailed manual for each command, type 'man <command name>'"
            );
            println!("");
            println!("{}", MANUAL);
        } else {
            for i in 1..self.get_tokens_length() {
                if COMMANDS.contains(&self.get_token(i)) {
                    println!("{}", manual_detail[&self.get_token(i)]);
                } else {
                    println!("Command {} not found", self.get_token(i).red());
                }
            }
        }
    }
    pub fn run_cat(&self) {
        for i in 1..self.get_tokens_length() {
            let mut fpath = self.current_dir_path.clone();
            fpath.push(self.get_token(i));
            let data = fs::read_to_string(fpath).expect("Unable to read file data");
            println!("{}", data);
        }
    }
    pub fn run_ls(&self) {
        if self.get_tokens_length() > 2 {
            print!("{}", "too many arguments".red());
        }

        let mut fpath = self.current_dir_path.clone();
        if self.get_tokens_length() > 1 {
            fpath.push(self.get_token(1));
        }

        if let Ok(entries) = fs::read_dir(fpath) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    println!("{}", file_name.to_string_lossy());
                }
            }
        } else {
            println!(
                "{}",
                format!("{}", "Failed to read directory contents".red())
            );
        }
    }
    pub fn run_cd(&mut self) {
        if self.get_tokens_length() > 2 {
            println!("{}", format!("{}", "Too many arguments".red()));
        } else if self.get_tokens_length() == 2 {
            let dest = self.get_token(1);
            if dest == ".." {
                self.current_dir_path = self.current_dir_path.parent().unwrap().to_path_buf();
            } else if dest.starts_with('/') {
                // absolute path
                match fs::symlink_metadata(dest) {
                    Ok(metadata) => {
                        if metadata.is_file() {
                            println!("File exists, but not a path. Can't change directory");
                        } else if metadata.is_dir() {
                            self.current_dir_path = PathBuf::from(dest.to_string());
                        } else {
                            println!("Not a file nor a directory");
                        }
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            } else if dest.starts_with("./") {
                // relative path
                let mut abs_path = self.current_dir_path.clone();
                abs_path.push(&dest.to_string().leak()[2..]);
                match fs::symlink_metadata(abs_path.clone()) {
                    Ok(metadata) => {
                        if metadata.is_file() {
                            println!("File exists, but not a path. Can't change directory");
                        } else if metadata.is_dir() {
                            self.current_dir_path = abs_path;
                        } else {
                            println!("Not a file nor a directory");
                        }
                    }
                    Err(err) => eprintln!("Error: {}", err),
                }
            } else {
                println!("{}", format!("{}", "No such file or directory".red()));
            }
        }
    }
}

fn main() {
    let mut vars = Variables::new();

    println!(
        r#"
    _____ _      _____   _                 _             _               _        
    / ____| |    |_   _| | |               | |           | |             | |       
   | |    | |      | |   | |__  _   _    __| | ___  _ __ | |__   __ _ ___| |_ __ _ 
   | |    | |      | |   | '_ \| | | |  / _` |/ _ \| '_ \| '_ \ / _` / __| __/ _` |
   | |____| |____ _| |_  | |_) | |_| | | (_| | (_) | | | | |_) | (_| \__ \ || (_| |
    \_____|______|_____| |_.__/ \__, |  \__,_|\___/|_| |_|_.__/ \__,_|___/\__\__,_|
                                 __/ |                                             
                                |___/                                             
    "#
    );
    println!("Made with ♥ using Rust");
    println!("Type man for list of commands");

    loop {
        vars.display_header();
        vars.input_and_preprocess();

        if vars.empty() {
            continue;
        }

        match vars.get_first_token() {
            "echo" => vars.run_echo(),
            "pwd" => {
                vars.run_pwd();
            }
            "cd" => {
                vars.run_cd();
            }
            "ls" => {
                vars.run_ls();
            }
            "cat" => {
                vars.run_cat();
            }
            "find" => {
                println!("to do doing find");
            }
            "grep" => {
                println!("to do doing grep");
            }
            "exit" | "quit" => {
                println!("Exiting CLI");
                std::process::exit(0);
            }
            "man" => {
                vars.run_man();
            }
            &_ => {
                println!(
                    "Command {} not found, see 'man' for help",
                    vars.get_first_token().red().bold()
                );
            }
        }
    }
}
