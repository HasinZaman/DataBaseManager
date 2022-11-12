use std::{fmt, env};

use mysql::{prelude::*, Opts, Conn, Row, Error};

#[derive(Debug)]
pub struct DataBase {
    host: String,
    port: String,
    name: String,
    username: String,
    password: String,
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

impl DataBase {
    pub fn new(host: String, port: String, name: String, username: String, password: String) -> DataBase {
        DataBase { host: host, port: port, name: name, username: username, password: password }
    }
    pub fn from_env() -> Option<DataBase> {
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

        Option::Some(
            DataBase::new(db_host, db_port, db_name, db_username, db_password)
        )
    }
    fn get_conn(&self) -> mysql::Conn {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        );

        println!("{}", url);

        let url: Opts = Opts::from_url(&url).unwrap();

        Conn::new(url).unwrap()
    }
    pub fn execute<E>(&self, command: &str, row_map: fn(row: Result<Row, Error>) -> E ) -> Result<Vec<E>, Error> {
        let mut conn = self.get_conn();

        let statement = conn.prep(command).unwrap();

        let rows: Vec<E>;
        match conn.exec_iter(&statement, ()){
            Ok(iter) => {
                rows = iter.map(row_map).collect();
            },
            Err(err) => return Err(err),
        }

        let _result = conn.close(statement);
        
        Ok(rows)
    }
}
impl fmt::Display for DataBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "host:{}::{}\ndb_name:{}\nuser_name:{}\npassword:{}",
            self.host,
            self.port,
            self.name,
            self.username,
            self.password
        )
    }
}