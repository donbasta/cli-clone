use colored::Colorize;
use std::fs;

use crate::cmd::CMD;

use super::{AppResult, Runnable};

pub struct Ls<'a> {
    vars: &'a mut CMD,
}

impl<'a> Runnable for Ls<'a> {
    fn run(&mut self) -> AppResult<()> {
        if self.vars.get_tokens_length() > 2 {
            return Err("Too many arguments for ls".to_string().into());
        }

        let mut fpath = self.vars.get_current_dir_path().clone();
        if self.vars.get_tokens_length() > 1 {
            fpath.push(self.vars.get_token(1));
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
            Err(err) => Err(Box::new(err)),
        }
    }
}

impl<'a> Ls<'a> {
    pub fn new(cmd: &'a mut CMD) -> Self {
        Self { vars: cmd }
    }
}
