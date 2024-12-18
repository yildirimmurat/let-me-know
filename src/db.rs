use rusqlite::{Connection, Result};

pub fn setup_db() -> Result<Connection> {
    let conn = Connection::open("db.sqlite3")?;

    apply_migrations(&conn)?;

    Ok(conn)
}

fn apply_migrations(conn: &Connection) -> Result<()> {
    create_users_table(conn)?;
    create_products_table(conn)?;

    Ok(())
}

fn create_users_table(conn: &Connection) -> Result<()> {
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

fn create_products_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (\
            id INTEGER PRIMARY KEY,\
            name TEXT NOT NULL,\
            api_url TEXT NOT NULL,\
            search_url TEXT NOT NULL,\
            tracking TEXT NOT NULL)",
        [],
    )?;

    Ok(())
}
