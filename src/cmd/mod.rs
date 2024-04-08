use std::env;
use std::io::{self};
use std::path::PathBuf;

use std::io::Write;

use colored::*;

use chrono::Local;

use crate::binaries::cat::Cat;
use crate::binaries::cd::Cd;
use crate::binaries::echo::Echo;
use crate::binaries::ls::Ls;
use crate::binaries::man::Man;
use crate::binaries::pwd::Pwd;
use crate::binaries::touch::Touch;
use crate::binaries::Runnable;

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

    pub fn run_binary(&mut self) {
        if self.empty() {
            return;
        }

        match self.get_first_token() {
            "echo" => {
                let _ = Echo::new(self).run();
            }
            "pwd" => {
                let _ = Pwd::new(self).run();
            }
            "cd" => match Cd::new(self).run() {
                Ok(_) => {}
                Err(err) => eprintln!("Error: {}", format!("{}", err).red()),
            },
            "ls" => match Ls::new(self).run() {
                Ok(_) => {}
                Err(err) => eprintln!("Error: {}", format!("{}", err).red()),
            },
            "cat" => match Cat::new(self).run() {
                Ok(_) => {}
                Err(err) => eprintln!("Error: {}", format!("{}", err).red()),
            },
            "find" => println!("to do doing find"),
            "grep" => println!("to do doing grep"),
            "exit" | "quit" => {
                println!("Exiting CLI");
                std::process::exit(0);
            }
            "touch" => match Touch::new(self).run() {
                Ok(_) => {}
                Err(err) => eprintln!("Error: {}", format!("{}", err).red()),
            },
            "man" => match Man::new(self).run() {
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
