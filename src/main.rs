use crate::cmd::CMD;

pub mod binaries;
pub mod cmd;
pub mod ui;

fn main() -> Result<(), String> {
    println!(
        r#"
    _____ _      _____   _                 _             _               _
    / ____| |    |_   _| | |               | |           | |             | |
   | |    | |      | |   | |__  _   _    __| | ___  _ __ | |__   __ _ ___| |_ __ _
   | |    | |      | |   | '_ \| | | |  / _` |/ _ \| '_ \| '_ \ / _` / __| __/ _` |
   | |____| |____ _| |_  | |_) | |_| | | (_| | (_) | | | | |_) | (_| \__ \ || (_| |
    \_____|______|_____| |_.__/ \__, |  \__,_|\___/|_| |_|_.__/ \__,_|___/\__\__,_|
                                 __/ |
                                |___/
    "#
    );
    println!("Made with ♥ using Rust");
    println!("Type 'man' (without the quote) for getting the list of commands");

    let mut cmd = CMD::new()?;
    cmd.run();
    Ok(())
}
