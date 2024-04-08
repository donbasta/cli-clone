use std::collections::HashMap;

use crate::cmd::CMD;

use super::Runnable;

pub struct Man<'a> {
    vars: &'a CMD,
}

const MANUAL: &str = "echo: repeats input
cat: concatenate files
ls: list directories
find: locate files or directories
grep: matches text in files
";

const COMMANDS: [&str; 10] = [
    "echo", "pwd", "cd", "ls", "find", "grep", "cat", "exit", "quit", "man",
];

impl<'a> Runnable for Man<'a> {
    fn run(&mut self) -> Result<(), String> {
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

        if self.vars.get_tokens_length() == 1 {
            println!(
                "{}",
                "For more detailed manual for each command, type 'man <command name>'"
            );
            println!("");
            println!("{}", MANUAL);
            Ok(())
        } else if self.vars.get_tokens_length() == 2 {
            if COMMANDS.contains(&self.vars.get_token(1)) {
                println!("{}", manual_detail[&self.vars.get_token(1)]);
                Ok(())
            } else {
                Err(format!("man: Command {} not found", self.vars.get_token(1)))
            }
        } else {
            Err(
                "man: too many arguments. Type 'man man' for more detailed information."
                    .to_string(),
            )
        }
    }
}

impl<'a> Man<'a> {
    pub fn new(cmd: &'a CMD) -> Self {
        Self { vars: cmd }
    }
}
