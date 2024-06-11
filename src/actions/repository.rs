use crate::actions::types::Runner;
use crate::database;
use sqlite::State;

pub fn get_current_version() -> String {
    let conn = database::connection::get_connection().unwrap();

    let query = "
        SELECT * FROM gh_version WHERE active = true LIMIT 1;
    ";

    let mut stmt = conn.prepare(query).unwrap();

    return match stmt.next().ok() {
        Some(State::Row) => stmt.read::<String, _>("version").unwrap(),
        _ => {
            println!("Current github action version not found");
            String::default()
        }
    };
}

pub fn set_current_version(version: String) {
    let conn = database::connection::get_connection().unwrap();

    let query = "
        UPDATE gh_version SET active = false;
    ";

    conn.execute(query).unwrap();

    let query = format!(
        "
            INSERT INTO gh_version (version, active)
            VALUES ('{}', true);
        ",
        version.clone(),
    );

    conn.execute(query).unwrap();

    println!("Current github action version saved: {}", version.clone());
}

pub fn find_active_runners() -> Vec<Runner> {
    let conn = database::connection::get_connection().unwrap();

    let query = "
        SELECT * FROM gh_runners WHERE active = true;
    ";

    let values: Vec<Runner> = conn
        .prepare(query)
        .unwrap()
        .into_iter()
        .filter_map(|row| row.ok())
        .map(|row| Runner {
            id: Some(row.read::<i64, _>("id")),
            name: Some(row.read::<&str, _>("name").to_string()),
            token: Some(row.read::<&str, _>("token").to_string()),
        })
        .collect();

    println!("Active runners found: {:?}", values.len());

    values
}

pub fn save_runner(runner: Runner) {
    let conn = database::connection::get_connection().unwrap();

    let query = format!(
        "
            INSERT INTO gh_runners (name, token)
            VALUES ('{}', '{}');
        ",
        runner.clone().name.unwrap().clone(),
        runner.clone().token.unwrap().clone(),
    );

    conn.execute(query).unwrap();

    println!("Runner saved: {:?}", runner.clone().name.clone());
}

pub fn delete_runners() {
    let conn = database::connection::get_connection().unwrap();

    let query = "
        DELETE FROM gh_runners WHERE active = true;
    ";

    conn.execute(query).unwrap();

    println!("All runners disabled");
}
