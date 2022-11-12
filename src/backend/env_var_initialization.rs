use std::env;
use std::io;

use super::data_base::DataBase;

pub fn start_up(_args: &Vec<String>) -> DataBase {
    let args: Vec<String> = env::args().collect();

    let mut db_data = get_env_var();

    if db_data.is_none() || args.contains(&String::from("update")) {
        set_env_var();
        db_data = get_env_var();
    }

    let db_data = db_data.unwrap();

    println!("DB_host:{:?}", db_data.db_host);
    println!("DB_port:{:?}", db_data.db_port);
    println!("DB_name:{:?}", db_data.db_name);
    println!("DB_username:{:?}", db_data.db_username);
    println!("DB_password:{:?}", db_data.db_password);

    db_data
}

fn set_env_var() {
    let env_vars = vec![
        "DB_host",
        "DB_port",
        "DB_name",
        "DB_username",
        "DB_password",
    ];

    for key in env_vars.iter() {
        println!("Enter any {:?}:", key);

        let mut user_input: String = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        env::set_var(key, user_input.trim_end_matches("\r\n"));
    }
}

macro_rules! env_var_to_variable {
    ($key : literal, $var : ident) => {
        match env::var($key) {
            Err(_err) => {
                println!("{} not assigned", $key);
                return Option::None;
            }
            Ok(ok) => $var = ok,
        }
    };
}

fn get_env_var() -> Option<DataBase> {
    let db_host: String;
    let db_port: String;
    let db_name: String;
    let db_username: String;
    let db_password: String;

    env_var_to_variable!("DB_host", db_host);
    env_var_to_variable!("DB_port", db_port);
    env_var_to_variable!("DB_name", db_name);
    env_var_to_variable!("DB_username", db_username);
    env_var_to_variable!("DB_password", db_password);

    Option::Some(DataBase {
        db_host: db_host,
        db_port: db_port,
        db_name: db_name,
        db_username: db_username,
        db_password: db_password,
    })
}
