use rusqlite::{params, Connection, Result};

pub fn insert_user(conn: &Connection, name: &str, email: &str, last_sent_date: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO users (name, email, last_sent_date) VALUES (?1, ?2, ?3)",
        params![name, email, last_sent_date],
    )?;

    Ok(())
}

pub fn get_all_users(conn: &Connection) -> Result<Vec<(String, String)>> {
    let mut stmt = conn.prepare("SELECT name, email FROM users")?;
    let users = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            ))
    })?;

    let mut result = Vec::new();
    for user in users {
        result.push(user?);
    }

    Ok(result)
}
