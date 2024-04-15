use std::fs;

use colored::Colorize;

use crate::cmd::CMD;

use super::{AppResult, Runnable};

pub struct Cat<'a> {
    vars: &'a CMD,
}

impl<'a> Runnable for Cat<'a> {
    fn run(&mut self) -> AppResult<()> {
        for i in 1..self.vars.get_tokens_length() {
            let mut fpath = self.vars.get_current_dir_path().clone();
            fpath.push(self.vars.get_token(i));
            match fs::read_to_string(fpath) {
                Ok(data) => println!("{}", data),
                Err(err) => eprintln!("Error: {}", format!("{}", err).red()),
            }
        }
        Ok(())
    }
}

impl<'a> Cat<'a> {
    pub fn new(cmd: &'a CMD) -> Self {
        Self { vars: cmd }
    }
}
