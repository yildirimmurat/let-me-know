use rusqlite::{params, Connection, Result};

pub struct User {
    pub name: String,
    pub email: String,
    pub last_sent_date: Option<String>, // Date is stored as a string, nullable
}
pub fn insert_user(conn: &Connection, name: &str, email: &str, last_sent_date: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO users (name, email, last_sent_date) VALUES (?1, ?2, ?3)",
        params![name, email, last_sent_date],
    )?;

    Ok(())
}

pub fn get_all_users(conn: &Connection) -> Result<Vec<User>> {
    let mut stmt = conn.prepare("SELECT name, email, last_sent_date FROM users")?;
    let users_iter = stmt.query_map([], |row| {
        Ok(User {
            name: row.get(0)?,
            email: row.get(1)?,
            last_sent_date: row.get(2)?,
        })
    })?;

    let mut users = Vec::new();
    for user in users_iter {
        users.push(user?);
    }

    Ok(users)
}
