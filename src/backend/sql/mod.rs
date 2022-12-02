use std::{fmt, env::VarError, ops::{Deref, DerefMut}};

use mysql::{Error, Row};
use regex::Regex;
use lazy_static::lazy_static;

use super::data_base::{DataBase, DatabaseExecute};

#[derive(Debug)]
pub enum SQLError{
    NotValidCMD,
    InvalidQuery{expected_variant: SQL},
    FailedToConnect(VarError),
    Execution(Error),
    Err(String)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DDL(pub String);
impl Deref for DDL {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for DDL{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DatabaseExecute for DDL{
    type RowError = SQLError;
    fn execute<T, F>(&self, row_map: F) -> Result<Vec<T>, SQLError> where F : Fn(Result<Row, Error>) -> T {
        let db = DataBase::from_env();
        match db {
            Ok(db) => {
                let tmp: Vec<T> = match db.execute(&*self, row_map){
                    Ok(val) => val,
                    Err(err) => return Err(SQLError::Execution(err)),
                };
                Ok(tmp)
            },
            Err(err) => {
                Err(SQLError::FailedToConnect(err))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QDL(pub String);
impl Deref for QDL {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for QDL{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DatabaseExecute for QDL{
    type RowError = SQLError;
    fn execute<T, F>(&self, row_map: F) -> Result<Vec<T>, SQLError> where F : Fn(Result<Row, Error>) -> T {
        let db = DataBase::from_env();
        match db {
            Ok(db) => {
                let tmp: Vec<T> = match db.execute(&*self, row_map){
                    Ok(val) => val,
                    Err(err) => return Err(SQLError::Execution(err)),
                };
                Ok(tmp)
            },
            Err(err) => {
                Err(SQLError::FailedToConnect(err))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QML(pub String);
impl Deref for QML {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for QML{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DatabaseExecute for QML{
    type RowError = SQLError;
    fn execute<T, F>(&self, row_map: F) -> Result<Vec<T>, SQLError> where F : Fn(Result<Row, Error>) -> T {
        let db = DataBase::from_env();
        match db {
            Ok(db) => {
                let tmp: Vec<T> = match db.execute(&*self, row_map){
                    Ok(val) => val,
                    Err(err) => return Err(SQLError::Execution(err)),
                };
                Ok(tmp)
            },
            Err(err) => {
                Err(SQLError::FailedToConnect(err))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DCL(pub String);
impl Deref for DCL {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for DCL{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DatabaseExecute for DCL{
    type RowError = SQLError;
    fn execute<T, F>(&self, row_map: F) -> Result<Vec<T>, SQLError> where F : Fn(Result<Row, Error>) -> T {
        let db = DataBase::from_env();
        match db {
            Ok(db) => {
                let tmp: Vec<T> = match db.execute(&*self, row_map){
                    Ok(val) => val,
                    Err(err) => return Err(SQLError::Execution(err)),
                };
                Ok(tmp)
            },
            Err(err) => {
                Err(SQLError::FailedToConnect(err))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SQL {
    //Data Definition Language
    Create(DDL),
    Alter(DDL),
    Drop(DDL),
    Truncate(DDL),

    //Querying Data Language
    Select(QDL),

    //Query Manipulation Language
    Insert(QML),
    Update(QML),
    Delete(QML),

    //Data Control language
    Grant(DCL),
    Revoke(DCL),
}

pub enum SQLLanguage{
    DDL,
    QDL,
    QML,
    DCL
}

macro_rules! SQL_Parse {
    ($output_variant: ident, $language: ident, $regex_expr: literal, $sql_cmd: expr) => {
        {
            lazy_static! {
                static ref REGEX: Regex = Regex::new($regex_expr).unwrap();
            };

            if REGEX.is_match(&$sql_cmd){
                return Ok(SQL::$output_variant($language($sql_cmd.to_string())));
            }
        }
    };
}

impl SQL {
    pub fn from(query: &str) -> Result<SQL, SQLError> {
        
        //Data Definition Language
        SQL_Parse!(Create, DDL, "^[Cc][Rr][Ee][Aa][Tt][Ee] .+", query);
        SQL_Parse!(Alter, DDL, "^[Aa][Ll][Tt][Ee][Rr] .+", query);
        SQL_Parse!(Drop, DDL, "^[Dd][Rr][Oo][Pp] .+", query);
        SQL_Parse!(Truncate, DDL, "^[Tt][Rr][Uu][Nn][Cc][Aa][Tt][Ee] .+", query);

        //Querying Data Language
        SQL_Parse!(Select, QDL, "^[Ss][Ee][Ll][Ee][Cc][Tt] .+", query);

        //Query Manipulation Language
        SQL_Parse!(Insert, QML, "^[Ii][Nn][Ss][Ee][Rr][Tt] .+", query);
        SQL_Parse!(Update, QML, "^[Uu][Pp][Dd][Aa][Tt][Ee] .+", query);
        SQL_Parse!(Delete, QML, "^[Dd][Ee][Ll][Ee][Tt][Ee] .+", query);

        //Data Control language
        SQL_Parse!(Grant, DCL, "^[Gg][Rr][Aa][Nn][Tt] .+", query);
        SQL_Parse!(Revoke, DCL, "^[Rr][Ee][Vv][Oo][Kk][Ee] .+", query);

        Result::Err(SQLError::NotValidCMD)
    }

    pub fn ddl(&self) -> Option<&DDL> {
        match self {
            SQL::Create(ddl) |
            SQL::Alter(ddl) |
            SQL::Drop(ddl) |
            SQL::Truncate(ddl) => Some(ddl),
            _ => None,
        }
    }
    pub fn ddl_mut(&mut self) -> Option<&mut DDL> {
        match self {
            SQL::Create(ddl) |
            SQL::Alter(ddl) |
            SQL::Drop(ddl) |
            SQL::Truncate(ddl) => Some(ddl),
            _ => None,
        }
    }

    pub fn qdl(&self) -> Option<&QDL> {
        match self {
            SQL::Select(qdl) => Some(qdl),
            _ => None,
        }
    }
    pub fn qdl_mut(&mut self) -> Option<&mut QDL> {
        match self {
            SQL::Select(qdl) => Some(qdl),
            _ => None,
        }
    }
    
    pub fn qml(&self) -> Option<&QML> {
        match self{
            SQL::Insert(cmd) |
            SQL::Update(cmd) |
            SQL::Delete(cmd) => Some(cmd),
            _ => None
        }
    }
    pub fn qml_mut(&mut self) -> Option<&mut QML> {
        match self{
            SQL::Insert(cmd) |
            SQL::Update(cmd) |
            SQL::Delete(cmd) => Some(cmd),
            _ => None
        }
    }

    pub fn dcl(&self) -> Option<&DCL> {
        match self {
            SQL::Grant(cmd) |
            SQL::Revoke(cmd) => Some(cmd),
            _ => None,
        }
    }
    pub fn dcl_mut(&mut self) -> Option<&mut DCL> {
        match self {
            SQL::Grant(cmd) |
            SQL::Revoke(cmd) => Some(cmd),
            _ => None,
        }
    }

    pub fn get_language(&self) -> SQLLanguage {
        match &self {
            SQL::Create(_) |
            SQL::Alter(_) |
            SQL::Drop(_) | 
            SQL::Truncate(_) => SQLLanguage::DDL,

            SQL::Select(_) => SQLLanguage::QDL,

            SQL::Insert(_) |
            SQL::Update(_) |
            SQL::Delete(_)  => SQLLanguage::QML,

            SQL::Grant(_)  |
            SQL::Revoke(_) => SQLLanguage::DCL,
        }
    }

    pub fn execute<E, F>(&self, row_map: F) -> Result<Vec<E>, SQLError> where F : Fn(Result<Row, Error>) -> E {
        let db = DataBase::from_env();

        match db {
            Ok(db) => {
                let tmp: Vec<E> = match db.execute(&self.to_string(), row_map){
                    Ok(val) => val,
                    Err(err) => return Err(SQLError::Execution(err)),
                };

                Ok(tmp)
            },
            Err(err) => {
                Err(SQLError::FailedToConnect(err))
            }
        }
    }
}

impl DatabaseExecute for SQL{
    type RowError = SQLError;
    fn execute<T, F>(&self, row_map: F) -> Result<Vec<T>, SQLError> where F : Fn(Result<Row, Error>) -> T {
        match self {
            SQL::Create(ddl) |
            SQL::Alter(ddl) |
            SQL::Drop(ddl) |
            SQL::Truncate(ddl) => ddl.execute(row_map),

            SQL::Select(qdl) => qdl.execute(row_map),

            SQL::Insert(qml) |
            SQL::Update(qml) |
            SQL::Delete(qml) => qml.execute(row_map),

            SQL::Grant(dcl) |
            SQL::Revoke(dcl) => dcl.execute(row_map)
        }
    }
}

impl fmt::Display for SQL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SQL::Create(cmd) |
            SQL::Alter(cmd) |
            SQL::Drop(cmd) | 
            SQL::Truncate(cmd) => write!(f, "{}", **cmd),

            SQL::Select(cmd) => write!(f, "{}", **cmd),

            SQL::Insert(cmd) |
            SQL::Update(cmd) |
            SQL::Delete(cmd)  => write!(f, "{}", **cmd),

            SQL::Grant(cmd)  |
            SQL::Revoke(cmd) => write!(f, "{}", **cmd),
        }
    }
}

mod tests{
    use super::*;
    //test parsing
    //Data Definition Language
    #[test]
    fn create_test_1() {
        let input = "CREATE TABLE MyTable (RowId INT64 NOT NULL, `Order` INT64 ) PRIMARY KEY (RowId)";

        let actual = SQL::from(input).unwrap();

        
        assert_eq!(
            actual, 
            SQL::Create(
                DDL(
                    input.to_string()
                )
            )
        );

        assert_eq!(
            actual.ddl(),
            Some(
                &DDL(
                    input.to_string()
                )
            )
        );
    }

    #[test]
    fn alter_test_1() {
        let input = "ALTER DATABASE database_id";

        let actual = SQL::from(input).unwrap();

        
        assert_eq!(
            actual, 
            SQL::Alter(
                DDL(
                    input.to_string()
                )
            )
        );

        assert_eq!(
            actual.ddl(),
            Some(
                &DDL(
                    input.to_string()
                )
            )
        );
    }

    #[test]
    fn drop_test_1() {
        let input = "DROP TABLE table_name";

        let actual = SQL::from(input).unwrap();

        
        assert_eq!(
            actual, 
            SQL::Drop(
                DDL(
                    input.to_string()
                )
            )
        );

        assert_eq!(
            actual.ddl(),
            Some(
                &DDL(
                    input.to_string()
                )
            )
        );
    }

    #[test]
    fn truncate_test_1() {
        let input = "TRUNCATE TABLE table_name";

        let actual = SQL::from(input).unwrap();

        
        assert_eq!(
            actual, 
            SQL::Truncate(
                DDL(
                    input.to_string()
                )
            )
        );

        assert_eq!(
            actual.ddl(),
            Some(
                &DDL(
                    input.to_string()
                )
            )
        );
    }
    //Query Data Language
    #[test]
    fn select_test_1() {
        let input = "SELECT 1 + 1";

        let actual = SQL::from(input).unwrap();

        
        assert_eq!(
            actual, 
            SQL::Select(
                QDL(
                    input.to_string()
                )
            )
        );

        assert_eq!(
            actual.qdl(),
            Some(
                &QDL(
                    input.to_string()
                )
            )
        );
    }

    //Query Manipulation Language
    
    #[test]
    fn insert_test_1() {
        let input = "INSERT INTO tbl_name (col1,col2) VALUES(15,col1*2)";

        let actual = SQL::from(input).unwrap();

        
        assert_eq!(
            actual, 
            SQL::Insert(
                QML(
                    input.to_string()
                )
            )
        );

        assert_eq!(
            actual.qml(),
            Some(
                &QML(
                    input.to_string()
                )
            )
        );
    }
    
    #[test]
    fn update_test_1() {
        let input = "UPDATE t1 SET col1 = col1 + 1, col2 = col1";

        let actual = SQL::from(input).unwrap();

        assert_eq!(
            actual, 
            SQL::Update(
                QML(
                    input.to_string()
                )
            )
        );

        assert_eq!(
            actual.qml(),
            Some(
                &QML(
                    input.to_string()
                )
            )
        );
    }
    
    #[test]
    fn delete_test_1() {
        let input = "DELETE FROM somelog WHERE user = 'jcole' ORDER BY timestamp_column LIMIT 1";

        let actual = SQL::from(input).unwrap();

        assert_eq!(
            actual, 
            SQL::Delete(
                QML(
                    input.to_string()
                )
            )
        );

        assert_eq!(
            actual.qml(),
            Some(
                &QML(
                    input.to_string()
                )
            )
        );
    }
    
    //Data Control Language
    #[test]
    fn grant_test_1() {
        let input = "GRANT ALL ON db1.* TO 'jeffrey'@'localhost'";

        let actual = SQL::from(input).unwrap();

        assert_eq!(
            actual, 
            SQL::Grant(
                DCL(
                    input.to_string()
                )
            )
        );

        assert_eq!(
            actual.dcl(),
            Some(
                &DCL(
                    input.to_string()
                )
            )
        );
    }
    
    #[test]
    fn revoke_test_1() {
        let input = "REVOKE INSERT ON *.* FROM 'jeffrey'@'localhost'";

        let actual = SQL::from(input).unwrap();

        assert_eq!(
            actual, 
            SQL::Revoke(
                DCL(
                    input.to_string()
                )
            )
        );

        assert_eq!(
            actual.dcl(),
            Some(
                &DCL(
                    input.to_string()
                )
            )
        );
    }
}