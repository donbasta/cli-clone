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

impl<'a> Runnable for Todo<'a> {
    fn run(&mut self) -> Result<(), String> {
        match Connection::open("./database.db") {
            Ok(conn) => {
                if let Err(err) = conn.execute(
                    "CREATE TABLE IF NOT EXISTS activity_2 (
                    id    INTEGER PRIMARY KEY,
                    name  TEXT NOT NULL,
                    date  TEXT NOT NULL,
                    description TEXT NOT NULL,
                    is_done BOOLEAN DEFAULT FALSE NOT NULL
                )",
                    (),
                ) {
                    return Err(err.to_string());
                }

                let act_1 = Activity {
                    name: "cari baju".to_string(),
                    date: "07/04/2024".to_string(),
                    description: "cari baju di shopee under 100rb buat acara".to_string(),
                    id: 0,
                    is_done: false,
                };

                if let Err(err) = conn.execute(
                    "INSERT INTO activity_2 (name, date, description) VALUES (?1, ?2, ?3)",
                    (&act_1.name, &act_1.date, &act_1.description),
                ) {
                    return Err(err.to_string());
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

impl<'a> Todo<'a> {
    pub fn new(cmd: &'a mut CMD) -> Self {
        Self { vars: cmd }
    }
}
