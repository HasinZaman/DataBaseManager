use std::{fmt, env::VarError, ops::{Deref, DerefMut}, fs::File, io::Read};

use mysql::{Error, Row};
use regex::Regex;
use lazy_static::lazy_static;

use super::data_base::{DataBase, DatabaseExecute};

mod file_insertion;

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
                let tmp: Vec<T> = match db.execute(&SQL::from(&*self).unwrap(), row_map){
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
                let tmp: Vec<T> = match db.execute(&SQL::from(&*self).unwrap(), row_map){
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
                let tmp: Vec<T> = match db.execute(&SQL::from(&*self).unwrap(), row_map){
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
                let tmp: Vec<T> = match db.execute(&SQL::from(&*self).unwrap(), row_map){
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
    Show(DDL),

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
        
        let query = match file_insertion::contents(query) {
            Ok(val) => val,
            Err(err) => return Err(SQLError::Err(err.to_string()))
        };

        //Data Definition Language
        SQL_Parse!(Create, DDL, "^[Cc][Rr][Ee][Aa][Tt][Ee] .+", query);
        SQL_Parse!(Alter, DDL, "^[Aa][Ll][Tt][Ee][Rr] .+", query);
        SQL_Parse!(Drop, DDL, "^[Dd][Rr][Oo][Pp] .+", query);
        SQL_Parse!(Truncate, DDL, "^[Tt][Rr][Uu][Nn][Cc][Aa][Tt][Ee] .+", query);
        SQL_Parse!(Show, DDL, "^[Ss][Hh][Oo][Ww] .+", query);

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
            SQL::Show(ddl) |
            SQL::Truncate(ddl) => Some(ddl),
            _ => None,
        }
    }
    pub fn ddl_mut(&mut self) -> Option<&mut DDL> {
        match self {
            SQL::Create(ddl) |
            SQL::Alter(ddl) |
            SQL::Drop(ddl) |
            SQL::Show(ddl) |
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
            SQL::Truncate(_) |
            SQL::Show(_) => SQLLanguage::DDL,

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
                let tmp: Vec<E> = match db.execute(&self, row_map){
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

    pub fn from_file(file_path: &str) -> Result<Vec<SQL>, std::io::Error> {
        let mut file: File = File::open(file_path)?;

        const BUFFER_SIZE: usize = 100;

        let mut buffer = [0; BUFFER_SIZE];
        let mut cmd : String = String::from("");

        let mut results: Vec<SQL> = Vec::new();
        
        loop {
            match file.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {

                    let buffer_iter = buffer[..n].iter();

                    for c in buffer_iter {
                        let c = *c as char;

                        cmd.push(c);

                        if c == '\n' {
                            if let Err(err) = SQL::extract_cmd(&mut cmd, &mut results) {
                            }
                        }
                    }
                    if let Err(err) = SQL::extract_cmd(&mut cmd, &mut results) {
                    }
                }
                Err(err) => {
                    return Err(err)
                }
            }
        }
        Ok(results)
    }

    fn extract_cmd(query_cmd: &mut String, results: &mut Vec<SQL>) -> Result<(), SQLError>  {
        //remove all comments
        lazy_static! {
            static ref COMMENT_CHECK_REGEX: Regex = Regex::new("--.*\n$").unwrap();
        };
        *query_cmd = COMMENT_CHECK_REGEX.replace_all(&*query_cmd, "").to_string();

        //check if just bunch of \n
        lazy_static! {
            static ref CMD_TRIM_REGEX: Regex = Regex::new("^[\r\n\t ]+").unwrap();
        };
        *query_cmd = CMD_TRIM_REGEX.replace_all(&*query_cmd, "").to_string();

        //check if command ends
        lazy_static! {
            static ref CMD_END_CHECK_REGEX: Regex = Regex::new(";\r?\n?$").unwrap();
        };

        if CMD_END_CHECK_REGEX.is_match(query_cmd) {
            let query_cmd_tmp = CMD_END_CHECK_REGEX.replace(query_cmd, "");
            
            match SQL::from(&query_cmd_tmp.to_string()) {
                Ok(val) => {
                    results.push(val);
                    query_cmd.clear();
                    return Ok(())
                },
                Err(err) => return Err(err)
            }
        }
        return Err(SQLError::Err(String::from("Failed to parse")))
    }

}

impl DatabaseExecute for SQL{
    type RowError = SQLError;
    fn execute<T, F>(&self, row_map: F) -> Result<Vec<T>, SQLError> where F : Fn(Result<Row, Error>) -> T {
        match self {
            SQL::Create(ddl) |
            SQL::Alter(ddl) |
            SQL::Drop(ddl) |
            SQL::Show(ddl) |
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
            SQL::Show(cmd) |
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
    use std::{fs, io::Write};
    //extracting commands from file
    #[test]
    fn file_parsing_multi_line_cmd() {
        let file_name = "file_parsing_multi_line_cmd.sql";
        {
            let mut file = File::create(file_name).unwrap();
            file.write(
                b"
                CREATE TABLE tag ( --multi line cmd
                    id INT AUTO_INCREMENT,
                    colour CHAR(6),
                    symbol VARCHAR(50) NOT NULL UNIQUE,
                    tag_type INT NOT NULL,
                    PRIMARY KEY(id)
                );
                
                
                SELECT \"\n\t\t\n\t\t\";--multi line but in a string
                "
            ).unwrap();
        }

        let actual = SQL::from_file(file_name);
        let expected = vec![
            SQL::from("CREATE TABLE tag (id INT AUTO_INCREMENT, colour CHAR(6), symbol VARCHAR(50) NOT NULL UNIQUE, tag_type INT NOT NULL, PRIMARY KEY(id))").unwrap(),
            SQL::from("SELECT \"\n\t\t\n\t\t\"").unwrap(),
        ];
        assert_eq!(
            actual.unwrap(),
            expected
        );

        fs::remove_file(file_name).unwrap();
    }

    #[test]
    fn file_parsing_multiple_cmds() {
        let file_name = "file_parsing_multiple_cmds.sql";
        {
            let mut file = File::create(file_name).unwrap();
            file.write(
                b"                
                --organizational tags\n
                INSERT INTO tag (colour, symbol, tag_type) VALUES (\"ffffff\", \"Web\", 2);
                INSERT INTO tag (colour, symbol, tag_type) VALUES (\"5CFFA1\", \"Shader\", 2);
                
                
                INSERT INTO tag (colour, symbol, tag_type) VALUES (\"FF5CBA\", \"GameJam\", 2);

                "
            ).unwrap();
        }

        let actual = SQL::from_file(file_name);
        let expected = vec![
            SQL::from("INSERT INTO tag (colour, symbol, tag_type) VALUES (\"ffffff\", \"Web\", 2)").unwrap(),
            SQL::from("INSERT INTO tag (colour, symbol, tag_type) VALUES (\"5CFFA1\", \"Shader\", 2)").unwrap(),
            SQL::from("INSERT INTO tag (colour, symbol, tag_type) VALUES (\"FF5CBA\", \"GameJam\", 2)").unwrap(),
        ];
        assert_eq!(
            actual.unwrap(),
            expected
        );

        fs::remove_file(file_name).unwrap();
    }

    #[test]
    fn file_insertion_1() {
        let file_name_1 = "file_insertion_1_1.sql";
        let file_name_2 = "file_insertion_1_2.txt";

        {
            let mut file = File::create(file_name_1).unwrap();
            file.write(
                &format!(
                    "INSERT INTO tag (colour, symbol, tag_type) VALUES (\"--file:({} as S)\", \"Web\", 2);",
                    file_name_2
                ).as_bytes()
            ).unwrap();
        }

        {
            let mut file = File::create(file_name_2).unwrap();
            file.write(
                b"ffffff"
            ).unwrap();
        }

        let actual = SQL::from_file(file_name_1);
        let expected = vec![
            SQL::from("INSERT INTO tag (colour, symbol, tag_type) VALUES (\"ffffff\", \"Web\", 2)").unwrap(),
        ];

        assert_eq!(
            actual.unwrap(),
            expected
        );

        fs::remove_file(file_name_1).unwrap();
        fs::remove_file(file_name_2).unwrap();
    }


    //testing SQL parsing
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