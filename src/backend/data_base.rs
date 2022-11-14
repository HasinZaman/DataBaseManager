use std::{fmt, env::{self, VarError}};

use mysql::{prelude::*, Opts, Conn, Row, Error};

macro_rules! load_env_var {
    ($key : literal) => {
        match env::var($key) {
            Err(err) => {
                return Err(err);
            }
            Ok(ok) => ok,
        }
    };
}

pub type RowMap<E> = fn(row: Result<Row, Error>) -> E;

#[derive(Debug)]
pub struct DataBase {
    host: String,
    port: String,
    name: String,
    username: String,
    password: String,
}

impl DataBase {
    pub fn new(host: String, port: String, name: String, username: String, password: String) -> DataBase {
        DataBase { host: host, port: port, name: name, username: username, password: password }
    }

    pub fn from_env() -> Result<DataBase, VarError> {
        Ok(DataBase {
            host: load_env_var!("DB_host"),
            port: load_env_var!("DB_port"),
            name: load_env_var!("DB_name"),
            username: load_env_var!("DB_username"),
            password: load_env_var!("DB_password"),
        })
    }

    fn get_conn(&self) -> mysql::Conn {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        );

        let url: Opts = Opts::from_url(&url).unwrap();

        Conn::new(url).unwrap()
    }
    
    pub fn execute<E>(&self, command: &str, row_map: RowMap<E> ) -> Result<Vec<E>, Error> {
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