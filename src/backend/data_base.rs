use std::{fmt, env::{self, VarError}};

use mysql::{prelude::*, Opts, Conn, Row, Error, TxOpts};

use super::sql::SQL;

pub trait DatabaseExecute{
    type RowError;

    fn execute<T,F>(&self, row_map: F) -> Result<Vec<T>, Self::RowError> where F : Fn(Result<Row, Error>) -> T;

}

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

    pub fn execute<E, F>(&self, command: &SQL, row_map: F ) -> Result<Vec<E>, Error> where F : FnMut(Result<Row, Error>) -> E{
        let mut conn = self.get_conn();

        let mut tx = conn.start_transaction(TxOpts::default()).unwrap();

        let statement = tx.prep(command.to_string()).unwrap();

        let mut rows: Vec<E> = Vec::new();

        let execute: Option<Error> = {
            let execute = tx.exec_iter(&statement, ());

            match execute {
                Ok(iter) => {
                    rows = iter.map(row_map).collect();
                    None
                },
                Err(err) => Some(err),
            }
        };

        if let Some(err) = execute {
            let _result = tx.rollback();
            return Err(err);
        }

        let _result = tx.commit();
        
        Ok(rows)
    }

    pub fn execute_multiple(&self, commands: &Vec<SQL>) -> Result<(), Error> {
        let mut conn = self.get_conn();

        let mut tx = conn.start_transaction(TxOpts::default()).unwrap();

        for sql in commands{
            let statement = tx.prep(sql.to_string()).unwrap();

            if let Err(err) = tx.exec_iter(&statement, ()) {
                //let _result = &tx.rollback();
                return Err(err)
            }
            
            let _result = tx.close(statement);
        }
        let _result = tx.commit();


        Ok(())
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