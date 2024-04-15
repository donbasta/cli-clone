use crate::cmd::CMD;
use std::{fs::OpenOptions, path::PathBuf};

use super::{AppResult, Runnable};

pub struct Touch<'a> {
    vars: &'a CMD,
}

impl<'a> Runnable for Touch<'a> {
    fn run(&mut self) -> AppResult<()> {
        if self.vars.get_tokens_length() == 1 {
            return Err(
                "touch: missing file operand. Type 'man touch' for more information"
                    .to_string()
                    .into(),
            );
        }

        let fpath = self.vars.get_token(1);
        if fpath.ends_with("/") {
            return Err("touch: can't create directory with touch"
                .to_string()
                .into());
        }

        match fpath {
            "." | ".." => Ok(()),
            &_ => {
                let absolute_path = match fpath {
                    fpath if fpath.starts_with("/") => PathBuf::from(fpath.to_string()),
                    &_ => {
                        let mut abs_path = self.vars.get_current_dir_path().clone();
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
                                    return Err(Box::new(err));
                                }
                            }
                        } else {
                            Err("touch: directory does not exist".to_string().into())
                        }
                    }
                    None => Err("touch: directory does not exist".to_string().into()),
                }
            }
        }
    }
}

impl<'a> Touch<'a> {
    pub fn new(cmd: &'a CMD) -> Self {
        Self { vars: cmd }
    }
}
