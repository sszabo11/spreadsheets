use std::collections::VecDeque;

use redis::Commands;

use crate::{cell::Cell, home::Sheet};

pub struct Database {
    conn: Option<redis::Connection>,
    client: redis::Client,
}

impl Database {
    pub fn new() -> redis::RedisResult<Self> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let conn = client.get_connection()?;

        Ok(Self {
            client,
            conn: Some(conn),
        })
    }

    pub fn get_connection(&mut self) -> redis::RedisResult<&mut redis::Connection> {
        if self.conn.is_none() {
            self.conn = Some(self.client.get_connection().unwrap());
        }
        Ok(self.conn.as_mut().unwrap())
    }

    pub fn create_sheet(&mut self, name: &str) -> redis::RedisResult<()> {
        let conn = self.get_connection().unwrap();
        let result: i32 = conn.lpush("spreadsheets", name).unwrap();

        Ok(())
    }

    pub fn get_sheets(&mut self) -> redis::RedisResult<Vec<Sheet>> {
        let conn = self.get_connection().unwrap();

        let sheets: Vec<String> = conn.lrange("spreadsheets", 0, -1)?;

        Ok(sheets
            .iter()
            .map(|name| Sheet {
                name: name.to_string(),
            })
            .collect())
    }

    pub fn delete_sheet(&mut self, sheet: &str) {
        let conn = self.get_connection().unwrap();

        let result: i32 = conn.lrem("spreadsheets", 1, sheet).unwrap();
    }

    pub fn get_cells(&mut self, sheet_id: &str) -> redis::RedisResult<Vec<(String, String)>> {
        let mut conn = self.get_connection().unwrap();
        let result: Vec<(String, String)> = conn.hgetall(sheet_id)?;

        //for (key, value) in result {
        //    println!("Key: {}", key);
        //    println!("Value: {}", value);
        //}

        Ok(result)
    }

    pub fn write_all_cells(
        &mut self,
        sheet_id: &str,
        cells: Vec<Vec<Cell>>,
    ) -> redis::RedisResult<()> {
        let mut conn = self.get_connection().unwrap();

        let mut pipe = redis::Pipeline::new();
        for row in 0..cells.len() {
            for col in 0..cells[0].len() {
                let value = cells[row][col].value.clone();
                if value.len() > 0 {
                    let key = format!("{}:{}", row, col);
                    let sheet_id = format!("spreadsheet:{}", sheet_id);
                    pipe.hset(sheet_id, key, value);
                }
            }
        }
        pipe.exec(&mut conn)?;

        Ok(())
    }
}
