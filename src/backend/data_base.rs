use crate::backend::relation::RelationMethods;

use std::{fmt, env::{self, VarError}, collections::HashMap};

use mysql::{prelude::*, Opts, Conn, Row, Error, TxOpts};

use super::{sql::{SQL, QDL}, relation::{Relation, paths::{get_dependency_tree, get_generation_path}}};

pub trait DatabaseExecute{
    type RowError;

    fn execute<T,F>(&self, row_map: F) -> Result<Vec<T>, Self::RowError> where F : FnMut(Result<Row, Error>) -> T;
}

/// An enum representing errors that may occur when interacting with a database.
#[derive(Debug)]
pub enum DatabaseError{
    /// An error occurred while attempting to load an environment variable.
    FailedToLoadENVVar(VarError),
    /// A general error occurred while interacting with the database.
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

/// A struct representing a database connection.
#[derive(Debug)]
pub struct DataBase {
    /// The hostname of the database.
    host: String,
    /// The port of the database.
    port: String,
    /// The name of the database.
    name: String,
    /// The username to use when connecting to the database.
    username: String,
    /// The password to use when connecting to the database.
    password: String,
}

impl DataBase {
    /// Creates a new `DataBase` with the given connection information.
    ///
    /// Returns `Some(DataBase)` if the connection was successful, or `None` if the connection failed.
    pub fn new(host: String, port: String, name: String, username: String, password: String) -> Option<DataBase> {
        let db = DataBase { host: host, port: port, name: name, username: username, password: password };

        match db.ping() {
            true => Some(db),
            false => None,
        }
    }

    /// Attempts to create a new `DataBase` by loading the necessary connection information from environment variables.
    ///
    /// Returns a `Result` with an error of type `DatabaseError` if the connection information could not be loaded or the connection failed.
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

    /// Gets a connection to the database using the connection information stored in this `DataBase`.
    fn get_conn(&self) -> mysql::Conn {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.name
        );

        let url: Opts = Opts::from_url(&url).unwrap();

        Conn::new(url).unwrap()
    }

    /// Tests the connection to the database by sending a "ping" query.
    ///
    /// Returns `true` if the ping was successful, or `false` if it failed.
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

    /// Executes a given `SQL` command on the database and maps the rows returned by the query to a type `E` using the provided function `row_map`.
    ///
    /// # Arguments
    /// 
    /// * `cmd` - `SQL` command that will be executed
    /// * `row_map` - `FnMut(Result<Row, Error>) -> E` is a function that maps a row to `E`
    /// 
    /// Returns a `Result` with an error of type `Error` if the query fails or there is a problem with the transaction.
    pub fn execute<E, F>(&self, cmd: &SQL, row_map: F ) -> Result<Vec<E>, Error> where F : FnMut(Result<Row, Error>) -> E{
        let mut conn = self.get_conn();

        let mut tx = conn.start_transaction(TxOpts::default())?;

        let mut rows: Vec<E> = Vec::new();

        let execute: Option<Error>;

        if let SQL::Select(QDL(cmd)) = cmd {
            execute = {
                let execute = tx.query_iter(cmd);
                match execute {
                    Ok(iter) => {
                        rows = iter.map(row_map).collect();
                        None
                    },
                    Err(err) => Some(err),
                }
            }
        }
        else {
            let statement = tx.prep(cmd.to_string());

            if let Err(err) = statement {
                log::error!("{:?}", err);
                return Err(err);
            }

            let statement = statement.unwrap();

            execute = {
                let execute = tx.exec_iter(&statement, ());

                match execute {
                    Ok(iter) => {
                        rows = iter.map(row_map).collect();
                        None
                    },
                    Err(err) => Some(err),
                }
            };
        }

        if let Some(err) = execute {
            let _result = tx.rollback();
            return Err(err);
        }

        

        let _result = tx.commit();
        
        Ok(rows)
    }

    /// Executes a list of `SQL` commands on the database as a single transaction.
    ///
    /// Returns a `Result` with an error of type `Error` if any of the queries fail or there is a problem with the transaction.
    pub fn execute_multiple(&self, commands: &Vec<SQL>) -> Result<(), Error> {
        let mut conn = self.get_conn();

        let mut tx = match conn.start_transaction(TxOpts::default()) {
            Ok(tx) => tx,
            Err(err) => {
                log::error!("Failed to start transaction - Err:{:?}", err);
                return Err(err);
            },
        };

        let mut fail : Option<Error> = None;

        for sql in commands{
            let statement = tx.prep(sql.to_string())?;

            match tx.exec_iter(&statement, ()) {
                Ok(_result) => {
                    //log::info!("Successfully cmd({}) - {:?}", sql.to_string(), result)
                },
                Err(err) => {
                    log::error!("Failed to execute command({}) - Err:{:?}", sql.to_string(), err);
                    fail = Some(err);
                    break;
                },
            }
            
            if let Err(err) = tx.close(statement) { 
                log::error!("Failed to close command({}) - Err:{:?}", sql.to_string(), err);
                fail = Some(err);
                break;
            }
        }
        
        match fail{
            Some(err) => {
                let _result = tx.rollback();
                return Err(err);
            },
            None => {
                
                let _result = tx.commit();
                return Ok(());
            },
        }

    }

    /// Returns vector of `SQL` to recreate the current state of the database
    pub fn get_snapshot(&self) -> Vec<SQL> {
        let relations = Relation::get_relations().unwrap();

        let dependencies = get_dependency_tree(&relations);

        let generation_order = get_generation_path(&relations, &dependencies);

        let relation_cmd: Vec<SQL> = generation_order.iter()
            .map(|index| {
                relations[*index].create().into()
            })
            .collect();

        let mut insertion_cmd: Vec<SQL> = generation_order.iter()
            .map(|index| {
                &relations[*index]
            })
            .filter(|relation| {
                match relation{
                    Relation::Table(_) => true,
                    Relation::View(_) => false,
                }
            })
            .map(|relation| {
                match relation {
                    Relation::Table(table) => table,
                    Relation::View(_) => panic!(),
                }
            })
            .flat_map(|table| {
                let result = table.select()
                    .execute(
                        |row| {
                        if let Err(_err) = row {
                            return None
                        }
                        let row = row.unwrap();

                        let mut attributes = HashMap::new();

                        row.columns()
                        .iter()
                        .map(|column| {
                            column.name_str().to_string()
                        }).zip(
                            row.unwrap().iter()
                                .map(|val| {
                                    val.as_sql(false).to_string()
                                })
                        ).for_each(|(column, value)| {
                            attributes.insert(
                                column,
                                value
                            );
                        });

                        Some(attributes)
                    }
                );

                if let Err(_err) = result {
                    todo!()
                }

                let values = result.unwrap();
                
                values.iter()
                .filter_map(|p| {
                    p.clone()
                })
                .map(|val| table.insert(&val).unwrap().into())
                .collect::<Vec<SQL>>()
            }).collect();
        
        let mut cmds = relation_cmd;

        cmds.append(&mut insertion_cmd);

        cmds
    }

    /// Returns Vector of `SQL` to delete all relations from database
    pub fn get_deletion_cmds(&self) -> Vec<SQL> {
        let relations = Relation::get_relations().unwrap();

        let dependencies = get_dependency_tree(&relations);

        let generation_order = get_generation_path(&relations, &dependencies);

        generation_order.iter()
            .rev()
            .map(|index| {
                &relations[*index]
            })
            .map(|relation| SQL::from(relation.drop()))
            .collect()
    }

    /// Deletes all relations from database
    pub fn delete_relations(&self) -> Result<(), Error> {
        self.execute_multiple(&self.get_deletion_cmds())
    }

    /// Updates the state of database to what is defined in the `new_state` parameter
    /// 
    /// # Arguments
    /// 
    /// * `new_state` - Vector of `SQL` commands to generate new state of database
    /// 
    /// Returns Error if there is a failure to connect or a failure to execute a SQL command from `new_state`
    pub fn rollback(&self, new_state: Vec<SQL>)  -> Result<(), Error> {
        let rollback_cmds : Vec<SQL> = vec![
            self.get_deletion_cmds(),
            new_state,
        ].iter()
        .flat_map(|sql| sql.clone())
        .collect();

        self.execute_multiple(&rollback_cmds)
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

#[cfg(test)]
mod tests{
    #[allow(unused_imports)]
    use std::{thread, time::Duration};

    #[allow(unused_imports)]
    use lazy_static::lazy_static;
    #[allow(unused_imports)]
    use regex::Regex;
    #[allow(unused_imports)]
    use serial_test::serial;

    #[allow(unused_imports)]
    use crate::{backend::sql::{SQL, DDL, QML}, test_tools::db_env::DbEnv};

    #[allow(unused_imports)]
    use super::DataBase;

    #[test]
    #[serial]
    fn execute_multiple_success() {
        let _env = DbEnv::new(
            vec![
                SQL::new("CREATE TABLE execute_multiple_success (col1 INT)").unwrap(),
                SQL::new("INSERT INTO execute_multiple_success (col1) VALUES (1)").unwrap(),
            ],
            vec![
                SQL::new("DROP TABLE execute_multiple_success").unwrap(),
            ]
        );

        let cmds = vec![
            SQL::new("INSERT INTO execute_multiple_success (col1) VALUES (2)").unwrap(),
            SQL::new("INSERT INTO execute_multiple_success (col1) VALUES (3)").unwrap(),
            SQL::new("INSERT INTO execute_multiple_success (col1) VALUES (4)").unwrap(),
        ];

        let db = DataBase::from_env().unwrap();

        let result = db.execute_multiple(&cmds);
        assert!(result.is_ok());

        let mut actual = db.execute(
            &SQL::new("SELECT * FROM execute_multiple_success").unwrap(),
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
    #[serial]
    fn execute_one_success() {
        let _env = DbEnv::new(
            vec![
                SQL::new("CREATE TABLE execute_one_success (col1 INT)").unwrap(),
                SQL::new("INSERT INTO execute_one_success (col1) VALUES (1)").unwrap(),
            ],
            vec![
                SQL::new("DROP TABLE execute_one_success").unwrap(),
            ]
        );

        let cmd = SQL::new("INSERT INTO execute_one_success (col1) VALUES (2)").unwrap();

        let db = DataBase::from_env().unwrap();

        let result = db.execute(&cmd, |_| ());
        assert!(result.is_ok());

        let mut actual = db.execute(
            &SQL::new("SELECT * FROM execute_one_success").unwrap(),
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
    #[serial]
    fn execute_multiple_fail() {
        let _env = DbEnv::new(
            vec![
                SQL::new("CREATE TABLE execute_multiple_fail (col1 Int(16), PRIMARY KEY(col1))").unwrap(),
                SQL::new("INSERT INTO execute_multiple_fail (col1) VALUES (1)").unwrap(),
            ],
            vec![
                SQL::new("DROP TABLE execute_multiple_fail").unwrap(),
            ]
        );

        //syntax error
        {
            let cmds = vec![
                SQL::new("INSERT INTO execute_multiple_fail (col1) VALUES (2)").unwrap(),
                SQL::new("INSERT INTO execute_multiple_fail (col1) VALUES (3))").unwrap(),
                SQL::new("INSERT INTO execute_multiple_fail (col1) VALUES (4)").unwrap(),
            ];
            let db = DataBase::from_env().unwrap();

            let result = db.execute_multiple(&cmds);
            assert!(result.is_err());

            let mut actual = db.execute(
                &SQL::new("SELECT * FROM execute_multiple_fail").unwrap(),
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
                SQL::new("INSERT INTO execute_multiple_fail (col1) VALUES (2)").unwrap(),
                SQL::new("INSERT INTO execute_multiple_fail (col1) VALUES (3)").unwrap(),
                SQL::new("INSERT INTO execute_multiple_fail (col1) VALUES (4)").unwrap(),
                SQL::new("INSERT INTO execute_multiple_fail (col1) VALUES (4)").unwrap(),
            ];
            let db = DataBase::from_env().unwrap();

            let result = db.execute_multiple(&cmds);
            assert!(result.is_err());

            let mut actual = db.execute(
                &SQL::new("SELECT * FROM execute_multiple_fail").unwrap(),
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
    #[serial]
    fn execute_one_fail() {
        let _env = DbEnv::new(
            vec![
                SQL::new("CREATE TABLE execute_one_fail (col1 INT)").unwrap(),
                SQL::new("INSERT INTO execute_one_fail (col1) VALUES (1)").unwrap(),
            ],
            vec![
                SQL::new("DROP TABLE execute_one_fail").unwrap(),
            ]
        );
        //syntax error
        {
            let cmd = SQL::new("INSERT INTO execute_one_fail (col1) VALUES (2))").unwrap();

            let db = DataBase::from_env().unwrap();

            let result = db.execute(&cmd, |_| ());
            assert!(result.is_err());

            let mut actual = db.execute(
                &SQL::new("SELECT * FROM execute_one_fail").unwrap(),
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
            let cmd = SQL::new("INSERT INTO execute_one_fail (col1) VALUES (1))").unwrap();

            let db = DataBase::from_env().unwrap();

            let result = db.execute(&cmd, |_| ());
            assert!(result.is_err());

            let mut actual = db.execute(
                &SQL::new("SELECT * FROM execute_one_fail").unwrap(),
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
    #[serial]
    fn get_snapshot_test_1() {
        let _env = DbEnv::new(
            vec![
                SQL::new("CREATE TABLE table_1_test (col1 Int(16))").unwrap(),
                SQL::new("INSERT INTO table_1_test (col1) VALUES (1)").unwrap(),
            ],
            vec![
                SQL::new("DROP TABLE table_1_test").unwrap(),
            ]
        );

        let actual: Vec<String> = DataBase::from_env()
            .unwrap()
            .get_snapshot()
            .iter()
            .filter(|cmd| {
                lazy_static!{
                    static ref TEST_ENV : Regex = Regex::new("table_1_test").unwrap();
                };
                TEST_ENV.is_match(&cmd.to_string())
            })
            .map(|cmd| cmd.to_string())
            .collect();

        assert_eq!(
            actual,
            vec![
                String::from("CREATE TABLE table_1_test (col1 int(16))"),
                String::from("INSERT INTO table_1_test(col1) VALUES (1)"),
            ]
        );
    }

    #[test]
    #[serial]
    fn get_snapshot_test_2() {
        let generation_cmd =  vec![
            SQL::new("CREATE TABLE patients (id INTEGER,first_name VARCHAR(255),last_name VARCHAR(255),date_of_birth DATE,gender CHAR(1),address VARCHAR(255), PRIMARY KEY(id))").unwrap(),
            SQL::new("CREATE TABLE appointments (id INTEGER,patient_id INTEGER,date DATE,time TIME,FOREIGN KEY (patient_id) REFERENCES patients(id), PRIMARY KEY(id))").unwrap(),
            SQL::new("CREATE TABLE medications (id INTEGER,patient_id INTEGER,name VARCHAR(255),dosage VARCHAR(255),FOREIGN KEY (patient_id) REFERENCES patients(id), PRIMARY KEY(id))").unwrap(),

            SQL::new("INSERT INTO patients(id, first_name, last_name, date_of_birth, gender, address) VALUES (1, 'John', 'Doe', '1970-01-01', 'M', '123 Main St')").unwrap(),
            SQL::new("INSERT INTO patients(id, first_name, last_name, date_of_birth, gender, address) VALUES (2, 'Jane', 'Doe', '1980-03-03', 'F', '456 Park Ave')").unwrap(),
            SQL::new("INSERT INTO patients(id, first_name, last_name, date_of_birth, gender, address) VALUES (3, 'Jack', 'Smith', '1990-05-05', 'M', '789 Maple St')").unwrap(),

            SQL::new("INSERT INTO appointments(id, patient_id, date, time) VALUES (1, 1, '2022-12-14', '09:00:00')").unwrap(),
            SQL::new("INSERT INTO appointments(id, patient_id, date, time) VALUES (2, 1, '2022-12-15', '10:00:00')").unwrap(),
            SQL::new("INSERT INTO appointments(id, patient_id, date, time) VALUES (3, 2, '2022-12-16', '11:00:00')").unwrap(),

            SQL::new("INSERT INTO medications(id, patient_id, name, dosage) VALUES (1, 1, 'Ibuprofen', '200mg')").unwrap(),
            SQL::new("INSERT INTO medications(id, patient_id, name, dosage) VALUES (2, 2, 'Aspirin', '325mg')").unwrap(),
            SQL::new("INSERT INTO medications(id, patient_id, name, dosage) VALUES (3, 3, 'Acetaminophen', '500mg')").unwrap(),
        ];
        let destruction_cmd = vec![
            SQL::new("DROP TABLE medications").unwrap(),
            SQL::new("DROP TABLE appointments").unwrap(),
            SQL::new("DROP TABLE patients").unwrap(),
        ];

        let _env = DbEnv::new(
            generation_cmd,
            destruction_cmd
        );

        let actual: Vec<String> = DataBase::from_env()
            .unwrap()
            .get_snapshot()
            .iter()
            .filter(|cmd| {
                lazy_static!{
                    static ref TEST_ENV : [Regex;3] = [
                        Regex::new("patients").unwrap(),
                        Regex::new("appointments").unwrap(),
                        Regex::new("medications").unwrap(),
                    ];
                };

                TEST_ENV.iter().any(|regex| regex.is_match(&cmd.to_string()))
            })
            .map(|cmd| cmd.to_string())
            .collect();

        let expected = vec![
            String::from("CREATE TABLE patients (id int(11) Not Null,first_name varchar(255),last_name varchar(255),date_of_birth date,gender char(1),address varchar(255), PRIMARY KEY(id))"),
            String::from("CREATE TABLE medications (id int(11) Not Null,patient_id int(11), FOREIGN KEY(patient_id) REFERENCES patients(id),name varchar(255),dosage varchar(255), PRIMARY KEY(id))"),
            String::from("CREATE TABLE appointments (id int(11) Not Null,patient_id int(11), FOREIGN KEY(patient_id) REFERENCES patients(id),date date,time time, PRIMARY KEY(id))"),

            String::from("INSERT INTO patients(id,first_name,last_name,date_of_birth,gender,address) VALUES (1,'John','Doe','1970-01-01','M','123 Main St')"),
            String::from("INSERT INTO patients(id,first_name,last_name,date_of_birth,gender,address) VALUES (2,'Jane','Doe','1980-03-03','F','456 Park Ave')"),
            String::from("INSERT INTO patients(id,first_name,last_name,date_of_birth,gender,address) VALUES (3,'Jack','Smith','1990-05-05','M','789 Maple St')"),

            String::from("INSERT INTO medications(id,patient_id,name,dosage) VALUES (1,1,'Ibuprofen','200mg')"),
            String::from("INSERT INTO medications(id,patient_id,name,dosage) VALUES (2,2,'Aspirin','325mg')"),
            String::from("INSERT INTO medications(id,patient_id,name,dosage) VALUES (3,3,'Acetaminophen','500mg')"),

            String::from("INSERT INTO appointments(id,patient_id,date,time) VALUES (1,1,'2022-12-14','009:00:00')"),
            String::from("INSERT INTO appointments(id,patient_id,date,time) VALUES (2,1,'2022-12-15','010:00:00')"),
            String::from("INSERT INTO appointments(id,patient_id,date,time) VALUES (3,2,'2022-12-16','011:00:00')"),
        ];

        actual.iter()
            .zip(expected.iter())
            .for_each(|(actual, expected)| assert_eq!(actual, expected));
    }

    #[test]
    #[serial]
    fn deletion_test() {
        let db = DataBase::from_env().unwrap();

        for cmd in db.get_snapshot() {
            println!("{:?}", cmd);
        }

        let _env = DbEnv::new(
            db.get_deletion_cmds(),
            db.get_snapshot()
        );

        let actual = db.get_snapshot();

        assert_eq!(actual, vec![])
    }

}