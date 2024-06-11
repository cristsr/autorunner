use self::super::connection;

pub fn sync_tables() {
    let conn = connection::get_connection().unwrap();

    let query = "
        CREATE TABLE IF NOT EXISTS gh_version (
            id INTEGER PRIMARY KEY,
            version TEXT NOT NULL,
            active BOOLEAN default 1,
            created_at datetime DEFAULT CURRENT_TIMESTAMP
        )
    ";
    conn.execute(query).unwrap();

    let query = "
        CREATE TABLE IF NOT EXISTS gh_runners (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            token TEXT NULL,
            active BOOLEAN default 1,
            created_at datetime DEFAULT CURRENT_TIMESTAMP
        )
    ";
    conn.execute(query).unwrap()
}
