use std::error::Error;

use rusqlite::Connection;

#[derive(Debug)]
pub struct Credential {
    id: Option<usize>,
    email: String,
    password: String,
}

impl Credential {
    pub fn new(id: Option<usize>, email: String, password: String) -> Self {
        Self {
            id,
            email,
            password,
        }
    }

    /// TODO
    /// - build query automatically
    pub fn read(connection: &Connection) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut select = connection.prepare("SELECT id, email, password FROM credentials;")?;
        let rows = select.query_map((), |row| {
            Ok(Credential {
                id: Some(row.get(0)?),
                email: row.get(1)?,
                password: row.get(2)?,
            })
        })?;
        Ok(rows.map(|row| row.unwrap()).collect())
    }

    /// TODO
    /// - build query automatically
    pub fn write(&self, connection: &Connection) -> Result<(), Box<dyn Error>> {
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

    pub fn delete(id: usize, connection: &Connection) -> Result<(), Box<dyn Error>> {
        if !(Credential::check_record_exists(id, connection)?) {
            todo!()
        }
        let delete = "
            DELETE FROM credentials WHERE id = ?1;
            ";
        connection.execute(delete, [&id])?;
        Ok(())
    }

    /// TODO
    /// - build query automatically
    pub fn update(&self, connection: &Connection) -> Result<(), Box<dyn Error>> {
        let id = self.id.unwrap();
        if !(Credential::check_record_exists(id, connection)?) {
            todo!()
        }
        let update = "
            UPDATE credentials
            SET email = ?1, password = ?2
            WHERE id = ?3;
            ";
        connection.execute(update, (&self.email, &self.password, &id))?;
        Ok(())
    }

    pub fn print(&self) {
        println!("{}|{}|{}", self.id.unwrap(), self.email, self.password);
    }

    fn check_record_exists(id: usize, connection: &Connection) -> rusqlite::Result<bool> {
        let mut select = connection.prepare(
            "
            SELECT 1 FROM credentials WHERE id = ?;
            ",
        )?;
        select.exists([&id])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_succeed() {
        let conn = Connection::open("examples/test.db").unwrap();
        let credentials = Credential::read(&conn).unwrap();
        let result = credentials.get(0).unwrap();
        assert_eq!(result.id, Some(1));
        assert_eq!(result.email, "test@example.com");
        assert_eq!(result.password, "123456");
    }

    #[test]
    fn test_read_fail() {
        let conn = Connection::open("examples/table_not_exist.db").unwrap();
        assert!(Credential::read(&conn).is_err());
    }

    #[test]
    fn test_write() {
        let credential = Credential {
            id: None,
            email: String::from("test@example.com"),
            password: String::from("123456"),
        };
        let conn = Connection::open_in_memory().unwrap();
        assert!(credential.write(&conn).is_ok());
    }

    #[test]
    fn test_update() {
        let credential = Credential {
            id: None,
            email: String::from("test@example.com"),
            password: String::from("123456"),
        };
        let conn = Connection::open_in_memory().unwrap();
        credential.write(&conn).unwrap();
        let credentials = Credential::read(&conn).unwrap();
        let credential = credentials.get(0).unwrap();
        let new_credential = Credential {
            id: credential.id,
            email: String::from("update@example.com"),
            password: String::from("78910"),
        };
        new_credential.update(&conn).unwrap();
        let credentials = Credential::read(&conn).unwrap();
        let result = credentials.get(0).unwrap();
        assert_eq!(result.email, String::from("update@example.com"));
        assert_eq!(result.password, String::from("78910"));
    }

    #[test]
    fn test_delete() {
        let credential = Credential {
            id: None,
            email: String::from("test@example.com"),
            password: String::from("123456"),
        };
        let conn = Connection::open_in_memory().unwrap();
        credential.write(&conn).unwrap();
        let credentials = Credential::read(&conn).unwrap();
        let result = credentials.get(0).unwrap();
        let id = result.id.unwrap();

        assert!(Credential::delete(id, &conn).is_ok());
    }

    #[test]
    fn test_check_record_exists() {
        let credential = Credential {
            id: None,
            email: String::from("test@example.com"),
            password: String::from("123456"),
        };
        let conn = Connection::open_in_memory().unwrap();
        credential.write(&conn).unwrap();
        let credentials = Credential::read(&conn).unwrap();
        let result = credentials.get(0).unwrap();
        let id = result.id.unwrap();
        let exist = Credential::check_record_exists(id, &conn).unwrap();
        assert!(exist);
    }
}
