pub mod cd;
pub mod echo;
pub mod ls;
pub mod pwd;

pub trait Runnable {
    fn run(&mut self) -> Result<(), String>;
}
