pub mod cat;
pub mod cd;
pub mod echo;
pub mod ls;
pub mod man;
pub mod pwd;
pub mod touch;

pub trait Runnable {
    fn run(&mut self) -> Result<(), String>;
}
