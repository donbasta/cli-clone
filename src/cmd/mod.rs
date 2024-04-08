use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self};
use std::path::PathBuf;

use std::collections::HashMap;
use std::io::Write;

use colored::*;

use chrono::Local;

use crate::binaries::cd::Cd;
use crate::binaries::echo::Echo;
use crate::binaries::ls::Ls;
use crate::binaries::pwd::Pwd;
use crate::binaries::Runnable;

const MANUAL: &str = "echo: repeats input
cat: concatenate files
ls: list directories
find: locate files or directories
grep: matches text in files
";

const COMMANDS: [&str; 10] = [
    "echo", "pwd", "cd", "ls", "find", "grep", "cat", "exit", "quit", "man",
];

pub struct CMD {
    raw_command: String,
    tokens: Vec<String>,
    chars: Vec<char>,
    current_dir_path: PathBuf,
}

impl Clone for CMD {
    fn clone(&self) -> Self {
        Self {
            raw_command: self.raw_command.clone(),
            tokens: self.tokens.clone(),
            chars: self.chars.clone(),
            current_dir_path: self.current_dir_path.clone(),
        }
    }
}

impl CMD {
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

    pub fn register_binaries() {
        todo!();
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

    pub fn get_chars(&self) -> &Vec<char> {
        return &self.chars;
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

    pub fn get_token(&self, idx: usize) -> &str {
        return &self.tokens[idx];
    }

    pub fn get_current_dir_path(&self) -> &PathBuf {
        return &self.current_dir_path;
    }

    pub fn set_current_dir_path(&mut self, path_buf: PathBuf) {
        self.current_dir_path = path_buf;
    }

    pub fn run_man(&self) -> Result<(), String> {
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
            Ok(())
        } else if self.get_tokens_length() == 2 {
            if COMMANDS.contains(&self.get_token(1)) {
                println!("{}", manual_detail[&self.get_token(1)]);
                Ok(())
            } else {
                Err(format!("man: Command {} not found", self.get_token(1)))
            }
        } else {
            Err(
                "man: too many arguments. Type 'man man' for more detailed information."
                    .to_string(),
            )
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
                            match OpenOptions::new()
                                .write(true)
                                .create(true)
                                .open(&absolute_path)
                            {
                                Ok(_) => Ok(()),
                                Err(err) => {
                                    return Err(err.to_string());
                                }
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

    pub fn run_binary(&mut self) {
        if self.empty() {
            return;
        }

        match self.get_first_token() {
            "echo" => {
                let mut echo_bin = Echo::new(self);
                let _ = echo_bin.run();
            }
            "pwd" => {
                let mut pwd_bin = Pwd::new(self);
                let _ = pwd_bin.run();
            }
            "cd" => {
                let mut cd_bin = Cd::new(self);
                match cd_bin.run() {
                    Ok(_) => {}
                    Err(err) => eprintln!("Error: {}", format!("{}", err).red()),
                }
            }
            "ls" => {
                let mut ls_bin = Ls::new(self);
                match ls_bin.run() {
                    Ok(_) => {}
                    Err(err) => eprintln!("Error: {}", format!("{}", err).red()),
                }
            }
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
            "man" => match self.run_man() {
                Ok(_) => {}
                Err(err) => eprintln!("Error: {}", format!("{}", err).red()),
            },
            &_ => {
                eprintln!(
                    "Error: Command {} not found, see 'man' for help",
                    self.get_first_token().red().bold()
                );
            }
        }
    }

    pub fn run(&mut self) {
        loop {
            self.display_header();
            self.input_and_preprocess();
            self.run_binary();
        }
    }
}
