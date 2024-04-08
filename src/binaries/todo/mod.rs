use crate::cmd::CMD;
use rusqlite::Connection;

use super::Runnable;

#[derive(Debug)]
struct Activity {
    id: u64,
    name: String,
    date: String,
    description: String,
    is_done: bool,
}

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
                date  TEXT NOT NULL,
                description TEXT NOT NULL,
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

    fn insert(&self, activity: &Activity) -> Result<(), String> {
        match Connection::open("./database.db") {
            Ok(conn) => {
                if let Err(err) = Self::create_table_if_not_exists(&conn, "activity_2") {
                    return Err(err);
                }

                if let Err(err) = conn.execute(
                    "INSERT INTO activity_2 (name, date, description) VALUES (?1, ?2, ?3)",
                    (&activity.name, &activity.date, &activity.description),
                ) {
                    return Err(err.to_string());
                }

                Ok(())
            }
            Err(err) => Err(err.to_string()),
        }
    }
    fn query_all(&self) -> Result<(), String> {
        match Connection::open("./database.db") {
            Ok(conn) => {
                if let Err(err) = Self::create_table_if_not_exists(&conn, "activity_2") {
                    return Err(err);
                }

                if let Ok(mut stmt) = conn.prepare("SELECT * FROM activity_2") {
                    if let Ok(activity_iter) = stmt.query_map([], |row| {
                        Ok(Activity {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            date: row.get(2)?,
                            description: row.get(3)?,
                            is_done: row.get(4)?,
                        })
                    }) {
                        for activity in activity_iter {
                            println!("Found person {:?}", activity.unwrap());
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
}

impl<'a> Runnable for Todo<'a> {
    fn run(&mut self) -> Result<(), String> {
        if self.vars.get_tokens_length() == 1 {
            return Ok(());
        }

        match self.vars.get_token(1) {
            "add" | "insert" => {
                if self.vars.get_tokens_length() < 4 {
                    return Err("Error: todo add, too little parameter.".to_string());
                }
                let dummy_act = Activity {
                    id: 0,
                    name: self.vars.get_token(2).to_string(),
                    date: self.vars.get_token(3).to_string(),
                    description: "test".to_string(),
                    is_done: false
                };
                return self.insert(&dummy_act);
            },
            "get" | "list" => self.query_all(),
            &_ => Err(format!("Error: Command {} not found for todo. Check 'man todo' for more detailed information.", self.vars.get_token(1)))
        }
    }
}

impl<'a> Todo<'a> {
    pub fn new(cmd: &'a mut CMD) -> Self {
        Self { vars: cmd }
    }
}
