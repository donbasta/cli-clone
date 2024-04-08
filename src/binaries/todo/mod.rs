use crate::cmd::CMD;
use rusqlite::Connection;

use colored::Colorize;

use super::Runnable;

#[derive(Debug)]
struct Activity {
    id: u64,
    name: String,
    is_done: bool,
}

const TABLE_NAME: &str = "Activity";
const DB_PATH: &str = "./database.db";

pub struct Todo<'a> {
    vars: &'a mut CMD,
}

impl<'a> Todo<'a> {
    fn create_table_if_not_exists(conn: &Connection, table_name: &str) -> Result<(), String> {
        if let Err(err) = conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} (
                id    INTEGER PRIMARY KEY,
                name  TEXT NOT NULL,
                is_done BOOLEAN DEFAULT FALSE NOT NULL
            )",
                table_name
            ),
            (),
        ) {
            return Err(err.to_string());
        }
        Ok(())
    }

    fn insert(&self, activity: Vec<Activity>) -> Result<(), String> {
        match Connection::open(DB_PATH) {
            Ok(conn) => {
                if let Err(err) = Self::create_table_if_not_exists(&conn, TABLE_NAME) {
                    return Err(err);
                }

                let query = format!("INSERT INTO {} (name) VALUES (?1)", TABLE_NAME);

                match conn.prepare(&query) {
                    Ok(mut stmt) => {
                        for act in activity.iter() {
                            if let Err(err) = stmt.execute([act.name.to_owned()]) {
                                return Err(err.to_string());
                            }
                        }
                        Ok(())
                    }
                    Err(err) => Err(err.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
    fn query_all(&self) -> Result<(), String> {
        match Connection::open(DB_PATH) {
            Ok(conn) => {
                if let Err(err) = Self::create_table_if_not_exists(&conn, TABLE_NAME) {
                    return Err(err);
                }

                if let Ok(mut stmt) = conn.prepare(&format!("SELECT * FROM {}", TABLE_NAME)) {
                    if let Ok(activity_iter) = stmt.query_map([], |row| {
                        Ok(Activity {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            is_done: row.get(2)?,
                        })
                    }) {
                        for activity in activity_iter {
                            match activity {
                                Ok(a) => {
                                    let name = a.name.red();
                                    let name_strikethrough = a.name.strikethrough().green();
                                    println!(
                                        "{}",
                                        format!(
                                            "{}\t{}",
                                            a.id,
                                            if a.is_done { name_strikethrough } else { name }
                                        )
                                    );
                                }
                                Err(_) => {}
                            }
                        }
                        return Ok(());
                    } else {
                        return Err("Error: Query Map Failed".to_string());
                    }
                } else {
                    return Err("Error: Query Statement Failed".to_string());
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
    fn update_done(&self, indices: Vec<u64>) -> Result<(), String> {
        match Connection::open(DB_PATH) {
            Ok(conn) => {
                if let Err(err) = Self::create_table_if_not_exists(&conn, TABLE_NAME) {
                    return Err(err);
                }

                let query = format!("UPDATE {} SET is_done = TRUE WHERE id = ?1", TABLE_NAME);

                match conn.prepare(&query) {
                    Ok(mut stmt) => {
                        for idx in indices.iter() {
                            if let Err(err) = stmt.execute([idx]) {
                                return Err(err.to_string());
                            }
                        }
                        Ok(())
                    }
                    Err(err) => Err(err.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
    fn update_undone(&self, indices: Vec<u64>) -> Result<(), String> {
        match Connection::open(DB_PATH) {
            Ok(conn) => {
                if let Err(err) = Self::create_table_if_not_exists(&conn, TABLE_NAME) {
                    return Err(err);
                }

                let query = format!("UPDATE {} SET is_done = FALSE WHERE id = ?1", TABLE_NAME);

                match conn.prepare(&query) {
                    Ok(mut stmt) => {
                        for idx in indices.iter() {
                            if let Err(err) = stmt.execute([idx]) {
                                return Err(err.to_string());
                            }
                        }
                        Ok(())
                    }
                    Err(err) => Err(err.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
    fn remove(&self, indices: Vec<u64>) -> Result<(), String> {
        match Connection::open(DB_PATH) {
            Ok(conn) => {
                if let Err(err) = Self::create_table_if_not_exists(&conn, TABLE_NAME) {
                    return Err(err);
                }

                let query = format!("DELETE FROM {} WHERE id = ?1", TABLE_NAME);

                match conn.prepare(&query) {
                    Ok(mut stmt) => {
                        for idx in indices.iter() {
                            if let Err(err) = stmt.execute([idx]) {
                                return Err(err.to_string());
                            }
                        }
                        Ok(())
                    }
                    Err(err) => Err(err.to_string()),
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
}

impl<'a> Runnable for Todo<'a> {
    fn run(&mut self) -> Result<(), String> {
        if self.vars.get_tokens_length() == 1 {
            println!("No command entered, see 'man todo' for more detailed information");
            return Ok(());
        }

        match self.vars.get_token(1) {
            "add" | "insert" => {
                if self.vars.get_tokens_length() == 2 {
                    return Err("Error: you need to add some tasks".to_string());
                }
                let mut activities_to_add : Vec<Activity> = Vec::new();
                for idx in 2..self.vars.get_tokens_length() {
                    activities_to_add.push(Activity{
                        id: 0, name: self.vars.get_token(idx).to_owned(), is_done: false
                    });
                }
                return self.insert(activities_to_add);
            },
            "get" | "list" => self.query_all(),
            "do" | "undo" => {
                if self.vars.get_tokens_length() == 2 {
                    return Err("Error: you need to provide the id of the task to be marked as done".to_string());
                }
                let mut activity_ids : Vec<u64> = Vec::new();
                for idx in 2..self.vars.get_tokens_length() {
                    match self.vars.get_token(idx).parse::<u64>() {
                        Ok(idx_int) => activity_ids.push(idx_int),
                        Err(_) => return Err("Error: id provided is not an integer; not valid".to_string()),
                    }
                }
                match self.vars.get_token(1) {
                    "do" => self.update_done(activity_ids),
                    "undo" => self.update_undone(activity_ids),
                    "remove" | "erase" | "delete" => self.remove(activity_ids),
                    &_ => Err("Error".to_string()),
                }
            }
            &_ => Err(format!("Error: Command {} not found for todo. Check 'man todo' for more detailed information.", self.vars.get_token(1)))
        }
    }
}

impl<'a> Todo<'a> {
    pub fn new(cmd: &'a mut CMD) -> Self {
        Self { vars: cmd }
    }
}
