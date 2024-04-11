use crate::cmd::CMD;

use self::{
    cat::Cat, cd::Cd, counter::Counter, echo::Echo, ls::Ls, man::Man, pwd::Pwd, todo::Todo,
    touch::Touch,
};

mod cat;
mod cd;
mod counter;
mod echo;
mod ls;
mod man;
mod pwd;
mod todo;
mod touch;

pub trait Runnable {
    fn run(&mut self) -> Result<(), String>;
}

pub enum BinEnum<'a> {
    Cat(Cat<'a>),
    Cd(Cd<'a>),
    Echo(Echo<'a>),
    Ls(Ls<'a>),
    Man(Man<'a>),
    Pwd(Pwd<'a>),
    Touch(Touch<'a>),
    Todo(Todo<'a>),
    Counter(Counter),
}

impl<'a> BinEnum<'a> {
    pub fn create(command: &str, vars: &'a mut CMD) -> Result<Self, String> {
        match command {
            "echo" => Ok(BinEnum::Echo(Echo::new(vars))),
            "pwd" => Ok(BinEnum::Pwd(Pwd::new(vars))),
            "ls" => Ok(BinEnum::Ls(Ls::new(vars))),
            "cat" => Ok(BinEnum::Cat(Cat::new(vars))),
            "cd" => Ok(BinEnum::Cd(Cd::new(vars))),
            "man" => Ok(BinEnum::Man(Man::new(vars))),
            "touch" => Ok(BinEnum::Touch(Touch::new(vars))),
            "todo" => Ok(BinEnum::Todo(Todo::new(vars))),
            "counter" => Ok(BinEnum::Counter(Counter::new())),
            &_ => Err(format!(
                "Error: Command {} not found, see 'man' for help",
                command
            )),
        }
    }
}

impl<'a> Runnable for BinEnum<'a> {
    fn run(&mut self) -> Result<(), String> {
        match self {
            BinEnum::Cat(cat) => cat.run(),
            BinEnum::Cd(cd) => cd.run(),
            BinEnum::Echo(echo) => echo.run(),
            BinEnum::Ls(ls) => ls.run(),
            BinEnum::Man(man) => man.run(),
            BinEnum::Pwd(pwd) => pwd.run(),
            BinEnum::Touch(touch) => touch.run(),
            BinEnum::Todo(todo) => todo.run(),
            BinEnum::Counter(counter) => counter.run(),
        }
    }
}
