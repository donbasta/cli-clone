use colored::Colorize;

use crate::cmd::CMD;

use super::{AppResult, Runnable};

pub struct Echo<'a> {
    vars: &'a CMD,
}

impl<'a> Runnable for Echo<'a> {
    fn run(&mut self) -> AppResult<()> {
        if self.vars.get_tokens_length() >= 2 {
            let mut itr = 4;
            while itr < self.vars.get_chars().len() && self.vars.get_chars()[itr] == ' ' {
                itr += 1;
            }
            let tmp: String = self.vars.get_chars().iter().collect();
            print!("{}", &tmp[itr..self.vars.get_chars().len()].green());
        }
        Ok(())
    }
}

impl<'a> Echo<'a> {
    pub fn new(cmd: &'a CMD) -> Self {
        Self { vars: cmd }
    }
}
