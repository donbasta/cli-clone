use std::env;
use std::fs::{self, File};
use std::io::{self};
use std::path::PathBuf;

use std::collections::HashMap;
use std::io::Write;

use colored::*;

use chrono::Local;

pub const MANUAL: &str = "echo: repeats input
cat: concatenate files
ls: list directories
find: locate files or directories
grep: matches text in files
";

pub const COMMANDS: [&str; 10] = [
    "echo", "pwd", "cd", "ls", "find", "grep", "cat", "exit", "quit", "man",
];

pub struct CMDVariables {
    raw_command: String,
    tokens: Vec<String>,
    chars: Vec<char>,
    current_dir_path: PathBuf,
}

impl CMDVariables {
    pub fn new() -> Result<Self, String> {
        match env::current_dir() {
            Ok(cur) => Ok(Self {
                raw_command: String::new(),
                tokens: Vec::new(),
                chars: Vec::new(),
                current_dir_path: cur,
            }),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn input_and_preprocess(&mut self) {
        self.raw_command = String::new();

        match io::stdin().read_line(&mut self.raw_command) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error when reading input: {}", err);
            }
        };

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
            format!(" {} ", Local::now().format("%Y-%m-%d %H:%M:%S").to_string())
                .black()
                .on_bright_yellow()
        );
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
            match fs::read_to_string(fpath) {
                Ok(data) => println!("{}", data),
                Err(err) => eprintln!("Error: {}", format!("{}", err).red()),
            }
        }
    }
    pub fn run_ls(&self) -> Result<(), String> {
        if self.get_tokens_length() > 2 {
            return Err("Too many arguments for ls".to_string());
        }

        let mut fpath = self.current_dir_path.clone();
        if self.get_tokens_length() > 1 {
            fpath.push(self.get_token(1));
        }

        match fs::read_dir(fpath) {
            Ok(entries) => {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name();
                        if entry.metadata().unwrap().is_dir() {
                            println!("{}", file_name.to_string_lossy().cyan());
                        } else if entry.metadata().unwrap().is_file() {
                            println!("{}", file_name.to_string_lossy().purple());
                        } else if entry.metadata().unwrap().is_symlink() {
                            println!("{}", file_name.to_string_lossy().yellow());
                        }
                    }
                }
                Ok(())
            }
            Err(err) => Err(err.to_string()),
        }
    }
    pub fn run_cd(&mut self) -> Result<(), String> {
        match self.get_tokens_length() {
            len if len > 2 => Err("Too many arguments".to_string()),
            len if len == 2 => {
                let dest = self.get_token(1);

                match dest {
                    "." => Ok(()),
                    ".." => match self.current_dir_path.parent() {
                        Some(c) => {
                            self.current_dir_path = c.to_path_buf();
                            Ok(())
                        }
                        None => Err("Can't move to parent directory".to_string()),
                    },
                    &_ => {
                        let absolute_path = match dest {
                            dest if dest.starts_with("/") => PathBuf::from(dest.to_string()),
                            dest if dest.starts_with("./") => {
                                let mut abs_path = self.current_dir_path.clone();
                                abs_path.push(&dest.to_string().leak()[2..]);
                                abs_path
                            }
                            dest if dest.starts_with("../") => {
                                let mut abs_path = self.current_dir_path.clone();
                                match abs_path.parent() {
                                    Some(c) => {
                                        abs_path = c.to_path_buf();
                                    }
                                    None => {
                                        return Err("Can't move to parent directory".to_string());
                                    }
                                }
                                abs_path.push(&dest.to_string().leak()[3..]);
                                abs_path
                            }
                            &_ => {
                                let mut abs_path = self.current_dir_path.clone();
                                abs_path.push(dest.to_string());
                                abs_path
                            }
                        };
                        match fs::symlink_metadata(absolute_path.clone()) {
                            Ok(metadata) => {
                                if metadata.is_file() {
                                    println!(
                                        "File exists, but not a directory. Can't change directory"
                                    );
                                } else if metadata.is_dir() {
                                    self.current_dir_path = absolute_path;
                                } else {
                                    println!("Not a file nor a directory");
                                }
                                Ok(())
                            }
                            Err(err) => Err(err.to_string()),
                        }
                    }
                }
            }
            _ => Ok(()),
        }
    }

    pub fn run_touch(&self) -> Result<(), String> {
        if self.get_tokens_length() == 1 {
            return Err(
                "touch: missing file operand. Type 'man touch' for more information".to_string(),
            );
        }

        let fpath = self.get_token(1);
        if fpath.ends_with("/") {
            return Err("touch: can't create directory with touch".to_string());
        }

        match fpath {
            "." | ".." => Ok(()),
            &_ => {
                let absolute_path = match fpath {
                    fpath if fpath.starts_with("/") => PathBuf::from(fpath.to_string()),
                    &_ => {
                        let mut abs_path = self.current_dir_path.clone();
                        abs_path.push(fpath.to_string());
                        abs_path
                    }
                };
                match absolute_path.parent() {
                    Some(parent_path) => {
                        if parent_path.exists() {
                            match File::create(absolute_path) {
                                Ok(_) => Ok(()),
                                Err(err) => Err(err.to_string()),
                            }
                        } else {
                            Err("touch: directory does not exist".to_string())
                        }
                    }
                    None => Err("touch: directory does not exist".to_string()),
                }
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            self.display_header();
            self.input_and_preprocess();

            if self.empty() {
                continue;
            }

            match self.get_first_token() {
                "echo" => self.run_echo(),
                "pwd" => self.run_pwd(),
                "cd" => match self.run_cd() {
                    Ok(_) => {}
                    Err(err) => eprintln!("Error: {}", format!("{}", err).red()),
                },
                "ls" => match self.run_ls() {
                    Ok(_) => {}
                    Err(err) => eprintln!("Error: {}", format!("{}", err).red()),
                },
                "cat" => self.run_cat(),
                "find" => println!("to do doing find"),
                "grep" => println!("to do doing grep"),
                "exit" | "quit" => {
                    println!("Exiting CLI");
                    std::process::exit(0);
                }
                "touch" => match self.run_touch() {
                    Ok(_) => {}
                    Err(err) => eprintln!("Error: {}", format!("{}", err).red()),
                },
                "man" => self.run_man(),
                &_ => {
                    eprintln!(
                        "Command {} not found, see 'man' for help",
                        self.get_first_token().red().bold()
                    );
                }
            }
        }
    }
}
