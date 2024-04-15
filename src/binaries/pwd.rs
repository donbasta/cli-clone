use colored::Colorize;

use crate::cmd::CMD;

use super::{AppResult, Runnable};

pub struct Pwd<'a> {
    vars: &'a CMD,
}

impl<'a> Runnable for Pwd<'a> {
    fn run(&mut self) -> AppResult<()> {
        println!(
            "{}",
            format!("{}", self.vars.get_current_dir_path().display()).cyan()
        );
        Ok(())
    }
}

impl<'a> Pwd<'a> {
    pub fn new(cmd: &'a CMD) -> Self {
        Self { vars: cmd }
    }
}
