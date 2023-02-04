use std::error::Error;

use rusqlite::Connection;

#[derive(Debug)]
pub struct Credential {
    email: String,
    password: String,
}

impl Credential {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }

    pub fn read(path: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        let connection = Connection::open(path)?;
        let mut select = connection.prepare("SELECT email, password FROM credentials;")?;
        let rows = select.query_map((), |row| {
            Ok(Credential {
                email: row.get(0)?,
                password: row.get(1)?,
            })
        })?;
        Ok(rows.map(|row| row.unwrap()).collect())
    }

    pub fn write(&self, path: &str) -> Result<(), Box<dyn Error>> {
        let connection = Connection::open(path)?;
        let create = "
            CREATE TABLE IF NOT EXISTS credentials(
              id INTEGER PRIMARY KEY AUTOINCREMENT,
              email TEXT NOT NULL,
              password TEXT NOT NULL
            );
            ";
        connection.execute(create, ())?;
        let insert = "
            INSERT INTO credentials (email, password) VALUES (?1, ?2);
            ";
        connection.execute(insert, (&self.email, &self.password))?;
        Ok(())
    }

    pub fn print(&self) {
        println!("{}|{}", self.email, self.password);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_read_succeed() {
        let credentials = Credential::read("examples/test.db").unwrap();
        let result = credentials.get(0).unwrap();
        assert_eq!(result.email, "test@example.com");
        assert_eq!(result.password, "123456");
    }

    #[test]
    fn test_read_fail() {
        assert!(Credential::read("examples/table_not_exist.db").is_err());
    }

    #[test]
    fn test_write() {
        let credential = Credential {
            email: String::from("test@example.com"),
            password: String::from("123456"),
        };
        let dummy_path = "dummy.db";
        assert!(credential.write(dummy_path).is_ok());
        fs::remove_file(dummy_path).unwrap();
    }
}
