use rusqlite::{Connection, Result};

pub fn setup_db() -> Result<Connection> {
    let conn = Connection::open("db.sqlite3")?;

    apply_migrations(&conn)?;

    Ok(conn)
}

fn apply_migrations(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (\
            id INTEGER PRIMARY KEY,\
            name TEXT NOT NULL,\
            email TEXT NOT NULL UNIQUE,\
            last_sent_date TEXT)",
        [],
    )?;

    Ok(())
}