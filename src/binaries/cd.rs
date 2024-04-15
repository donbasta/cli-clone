use std::{fs, path::PathBuf};

use crate::cmd::CMD;

use super::{AppResult, Runnable};

pub struct Cd<'a> {
    vars: &'a mut CMD,
}

impl<'a> Runnable for Cd<'a> {
    fn run(&mut self) -> AppResult<()> {
        match self.vars.get_tokens_length() {
            len if len > 2 => Err("Too many arguments".to_string().into()),
            len if len == 2 => {
                let dest = self.vars.get_token(1);

                match dest {
                    "." => Ok(()),
                    ".." => match self.vars.get_current_dir_path().parent() {
                        Some(c) => {
                            self.vars.set_current_dir_path(c.to_path_buf());
                            Ok(())
                        }
                        None => Err("Can't move to parent directory".to_string().into()),
                    },
                    &_ => {
                        let absolute_path = match dest {
                            dest if dest.starts_with("/") => PathBuf::from(dest.to_string()),
                            dest if dest.starts_with("./") => {
                                let mut abs_path = self.vars.get_current_dir_path().clone();
                                abs_path.push(&dest.to_string().leak()[2..]);
                                abs_path
                            }
                            dest if dest.starts_with("../") => {
                                let mut abs_path = self.vars.get_current_dir_path().clone();
                                match abs_path.parent() {
                                    Some(c) => {
                                        abs_path = c.to_path_buf();
                                    }
                                    None => {
                                        return Err("Can't move to parent directory"
                                            .to_string()
                                            .into());
                                    }
                                }
                                abs_path.push(&dest.to_string().leak()[3..]);
                                abs_path
                            }
                            &_ => {
                                let mut abs_path = self.vars.get_current_dir_path().clone();
                                abs_path.push(dest.to_string());
                                abs_path
                            }
                        };
                        match fs::symlink_metadata(&absolute_path) {
                            Ok(metadata) => {
                                if metadata.is_file() {
                                    println!(
                                        "File exists, but not a directory. Can't change directory"
                                    );
                                } else if metadata.is_dir() {
                                    self.vars.set_current_dir_path(absolute_path);
                                } else {
                                    println!("Not a file nor a directory");
                                }
                                Ok(())
                            }
                            Err(err) => Err(Box::new(err)),
                        }
                    }
                }
            }
            _ => Ok(()),
        }
    }
}

impl<'a> Cd<'a> {
    pub fn new(cmd: &'a mut CMD) -> Self {
        Self { vars: cmd }
    }
}
