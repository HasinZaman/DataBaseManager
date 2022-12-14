use std::{fmt, env::{self, VarError}};

use mysql::{prelude::*, Opts, Conn, Row, Error, TxOpts};

use super::{sql::{SQL, QDL}, relation::{Relation, paths::{get_dependency_graph, get_generation_path}}};

pub trait DatabaseExecute{
    type RowError;

    fn execute<T,F>(&self, row_map: F) -> Result<Vec<T>, Self::RowError> where F : FnMut(Result<Row, Error>) -> T;
}

#[derive(Debug)]
pub enum DatabaseError{
    FailedToLoadENVVar(VarError),
    Error(String)
}

macro_rules! load_env_var {
    ($key : literal) => {
        match env::var($key) {
            Err(err) => {
                return Err(DatabaseError::FailedToLoadENVVar(err));
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
    pub fn new(host: String, port: String, name: String, username: String, password: String) -> Option<DataBase> {
        let db = DataBase { host: host, port: port, name: name, username: username, password: password };

        match db.ping() {
            true => Some(db),
            false => None,
        }
    }

    pub fn from_env() -> Result<DataBase, DatabaseError> {

        let db = DataBase::new(
            load_env_var!("DB_host"),
            load_env_var!("DB_port"),
            load_env_var!("DB_name"),
            load_env_var!("DB_username"),
            load_env_var!("DB_password"),
        );

        match db {
            Some(db) => Ok(db),
            None => Err(DatabaseError::Error("Failed to connect".into())),
        }
    }

    fn get_conn(&self) -> mysql::Conn {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        );

        let url: Opts = Opts::from_url(&url).unwrap();

        Conn::new(url).unwrap()
    }

    pub fn ping(&self) -> bool {
        let result = self.execute(
            &SQL::Select(QDL(format!("SELECT 1"))), 
            |row| {
                match row{
                    Ok(_) => true,
                    Err(_) => false,
                }
            });
        
        match result {
            Ok(val) => val == vec![true],
            Err(_) => false,
        }
    }

    pub fn execute<E, F>(&self, command: &SQL, row_map: F ) -> Result<Vec<E>, Error> where F : FnMut(Result<Row, Error>) -> E{
        let mut conn = self.get_conn();

        let mut tx = conn.start_transaction(TxOpts::default())?;

        let statement = tx.prep(command.to_string())?;

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

        let mut tx = conn.start_transaction(TxOpts::default())?;

        for sql in commands{
            let statement = tx.prep(sql.to_string())?;

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

mod tests{
    use std::{fs::{File, self}, io::Write};

    use crate::backend::sql::SQL;

    use super::DataBase;

    struct FileEnv{
        file_name: String
    }
    impl FileEnv{
        pub fn new(file_name: &str, content: &str) -> FileEnv {

            let mut file = File::create(&file_name).unwrap();
            file.write(&content.as_bytes()).unwrap();

            FileEnv{ file_name: file_name.to_string() }
        }
    }
    impl Drop for FileEnv{
        fn drop(&mut self) {
            println!("Dropping {}", &self.file_name);
            fs::remove_file(&self.file_name).unwrap();
        }
    }

    struct DbEnv{
        undo: Vec<SQL>
    }
    impl DbEnv{
        pub fn new(set_up: Vec<SQL>, undo: Vec<SQL>) -> DbEnv {
            let tmp = DbEnv { undo: undo };
            
            let db = DataBase::from_env().unwrap();

            db.execute_multiple(&set_up).unwrap();

            tmp
        }
    }
    impl Drop for DbEnv{
        fn drop(&mut self) {
            let db = DataBase::from_env().unwrap();

            db.execute_multiple(&self.undo).unwrap();
        }
    }

    #[test]
    fn execute_multiple_success() {
        let _env = DbEnv::new(
            vec![
                SQL::from("CREATE TABLE execute_multiple_success (col1 INT)").unwrap(),
                SQL::from("INSERT INTO execute_multiple_success (col1) VALUES (1)").unwrap(),
            ],
            vec![
                SQL::from("DROP TABLE execute_multiple_success").unwrap(),
            ]
        );

        let cmds = vec![
            SQL::from("INSERT INTO execute_multiple_success (col1) VALUES (2)").unwrap(),
            SQL::from("INSERT INTO execute_multiple_success (col1) VALUES (3)").unwrap(),
            SQL::from("INSERT INTO execute_multiple_success (col1) VALUES (4)").unwrap(),
        ];

        let db = DataBase::from_env().unwrap();

        let result = db.execute_multiple(&cmds);
        assert!(result.is_ok());

        let mut actual = db.execute(
            &SQL::from("SELECT * FROM execute_multiple_success").unwrap(),
            |row| {
                let row = row.unwrap();

                let val: i32 = row.get(0).unwrap();
                
                val
            }
        ).unwrap();

        actual.sort();

        assert_eq!(
            actual,
            vec![1,2,3,4]
        );
    }

    #[test]
    fn execute_one_success() {
        let _env = DbEnv::new(
            vec![
                SQL::from("CREATE TABLE execute_one_success (col1 INT)").unwrap(),
                SQL::from("INSERT INTO execute_one_success (col1) VALUES (1)").unwrap(),
            ],
            vec![
                SQL::from("DROP TABLE execute_one_success").unwrap(),
            ]
        );

        let cmd = SQL::from("INSERT INTO execute_one_success (col1) VALUES (2)").unwrap();

        let db = DataBase::from_env().unwrap();

        let result = db.execute(&cmd, |_| ());
        assert!(result.is_ok());

        let mut actual = db.execute(
            &SQL::from("SELECT * FROM execute_one_success").unwrap(),
            |row| {
                let row = row.unwrap();

                let val: i32 = row.get(0).unwrap();
                
                val
            }
        ).unwrap();

        actual.sort();

        assert_eq!(
            actual,
            vec![1,2]
        );
    }
    
    #[test]
    fn execute_multiple_fail() {
        let _env = DbEnv::new(
            vec![
                SQL::from("CREATE TABLE execute_multiple_fail (col1 INT, PRIMARY KEY(col1))").unwrap(),
                SQL::from("INSERT INTO execute_multiple_fail (col1) VALUES (1)").unwrap(),
            ],
            vec![
                SQL::from("DROP TABLE execute_multiple_fail").unwrap(),
            ]
        );

        //syntax error
        {
            let cmds = vec![
                SQL::from("INSERT INTO execute_multiple_fail (col1) VALUES (2)").unwrap(),
                SQL::from("INSERT INTO execute_multiple_fail (col1) VALUES (3))").unwrap(),
                SQL::from("INSERT INTO execute_multiple_fail (col1) VALUES (4)").unwrap(),
            ];
            let db = DataBase::from_env().unwrap();

            let result = db.execute_multiple(&cmds);
            assert!(result.is_err());

            let mut actual = db.execute(
                &SQL::from("SELECT * FROM execute_multiple_fail").unwrap(),
                |row| {
                    let row = row.unwrap();

                    let val: i32 = row.get(0).unwrap();
                    
                    val
                }
            ).unwrap();

            actual.sort();

            assert_eq!(
                actual,
                vec![1]
            );
        }
        //db constraint violation
        {
            let cmds = vec![
                SQL::from("INSERT INTO execute_multiple_fail (col1) VALUES (2)").unwrap(),
                SQL::from("INSERT INTO execute_multiple_fail (col1) VALUES (3)").unwrap(),
                SQL::from("INSERT INTO execute_multiple_fail (col1) VALUES (4)").unwrap(),
                SQL::from("INSERT INTO execute_multiple_fail (col1) VALUES (4)").unwrap(),
            ];
            let db = DataBase::from_env().unwrap();

            let result = db.execute_multiple(&cmds);
            assert!(result.is_err());

            let mut actual = db.execute(
                &SQL::from("SELECT * FROM execute_multiple_fail").unwrap(),
                |row| {
                    let row = row.unwrap();

                    let val: i32 = row.get(0).unwrap();
                    
                    val
                }
            ).unwrap();

            actual.sort();

            assert_eq!(
                actual,
                vec![1]
            );
        }
        

        
    }

    #[test]
    fn execute_one_fail() {
        let _env = DbEnv::new(
            vec![
                SQL::from("CREATE TABLE execute_one_fail (col1 INT)").unwrap(),
                SQL::from("INSERT INTO execute_one_fail (col1) VALUES (1)").unwrap(),
            ],
            vec![
                SQL::from("DROP TABLE execute_one_fail").unwrap(),
            ]
        );
        //syntax error
        {
            let cmd = SQL::from("INSERT INTO execute_one_fail (col1) VALUES (2))").unwrap();

            let db = DataBase::from_env().unwrap();

            let result = db.execute(&cmd, |_| ());
            assert!(result.is_err());

            let mut actual = db.execute(
                &SQL::from("SELECT * FROM execute_one_fail").unwrap(),
                |row| {
                    let row = row.unwrap();

                    let val: i32 = row.get(0).unwrap();
                    
                    val
                }
            ).unwrap();

            actual.sort();

            assert_eq!(
                actual,
                vec![1]
            );
        }
        //constraint violation
        {
            let cmd = SQL::from("INSERT INTO execute_one_fail (col1) VALUES (1))").unwrap();

            let db = DataBase::from_env().unwrap();

            let result = db.execute(&cmd, |_| ());
            assert!(result.is_err());

            let mut actual = db.execute(
                &SQL::from("SELECT * FROM execute_one_fail").unwrap(),
                |row| {
                    let row = row.unwrap();

                    let val: i32 = row.get(0).unwrap();
                    
                    val
                }
            ).unwrap();

            actual.sort();

            assert_eq!(
                actual,
                vec![1]
            );
        }
    }
    
}