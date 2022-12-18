use std::{fmt, ops::{Deref, DerefMut}, fs::File, io::{Read, Write}};

use mysql::{Error, Row};
use regex::Regex;
use lazy_static::lazy_static;

use super::data_base::{DataBase, DatabaseExecute, DatabaseError};

mod file_insertion;

/// Represents possible errors that can occur when executing a SQL command.
#[derive(Debug)]
pub enum SQLError{
    /// The command is not a valid SQL command.
    NotValidCMD,
    /// The command is not a valid variant of the `SQL` enum.
    InvalidQuery{expected_variant: SQL},
    /// There was an error connecting to the database.
    FailedToConnect(DatabaseError),
    /// There was an error executing the command on the database.
    Execution(Error),
    /// There was a general error with the command.
    Err(String)
}

/// Represents a data definition language (DDL) SQL command.
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
    fn execute<T, F>(&self, row_map: F) -> Result<Vec<T>, SQLError> where F : FnMut(Result<Row, Error>) -> T {
        let db = DataBase::from_env();
        match db {
            Ok(db) => {
                //let sql = SQL::from(self);
                let tmp: Vec<T> = match db.execute(&self.into(), row_map){
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
impl From<DDL> for SQL{
    fn from(ddl: DDL) -> Self {
        let sql = SQL::new(&ddl)
            .unwrap_or_else(|_| panic!("\"{}\" is an invalid ddl state", *ddl));

        match sql {
            SQL::Create(_) |
            SQL::Alter(_) |
            SQL::Drop(_) |
            SQL::Truncate(_) |
            SQL::Show(_) => sql,

            _=> panic!("\"{}\" is an invalid ddl state", *ddl)
        }
    }
}
impl From<&DDL> for SQL{
    fn from(ddl: &DDL) -> Self {
        let sql = SQL::new(&ddl)
            .unwrap_or_else(|_| panic!("\"{}\" is an invalid ddl state", **ddl));

        match sql {
            SQL::Create(_) |
            SQL::Alter(_) |
            SQL::Drop(_) |
            SQL::Truncate(_) |
            SQL::Show(_) => sql,

            _=> panic!("\"{}\" is an invalid ddl state", **ddl)
        }
    }
}
impl From<&DDL> for SQLLanguage{
    fn from(_: &DDL) -> Self {
        SQLLanguage::DDL
    }
}

/// Represents a data query language (DQL) SQL command.
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
    fn execute<T, F>(&self, row_map: F) -> Result<Vec<T>, SQLError> where F : FnMut(Result<Row, Error>) -> T {
        let db = DataBase::from_env();
        match db {
            Ok(db) => {
                let tmp: Vec<T> = match db.execute(&self.into(), row_map){
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
impl From<QDL> for SQL{
    fn from(qdl: QDL) -> Self {
        let sql = SQL::new(&qdl)
            .unwrap_or_else(|_| panic!("\"{}\" is an invalid qdl state", *qdl));

        match sql {
            SQL::Select(_) => sql,

            _=> panic!("\"{}\" is an invalid qdl state", *qdl)
        }
    }
}
impl From<&QDL> for SQL{
    fn from(qdl: &QDL) -> Self {
        let sql = SQL::new(&qdl)
            .unwrap_or_else(|_| panic!("\"{}\" is an invalid qdl state", **qdl));

        match sql {
            SQL::Select(_) => sql,

            _=> panic!("\"{}\" is an invalid qdl state", **qdl)
        }
    }
}
impl From<&QDL> for SQLLanguage{
    fn from(_: &QDL) -> Self {
        SQLLanguage::QDL
    }
}
/// Represents a data modification language (DML) SQL command.
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
    fn execute<T, F>(&self, row_map: F) -> Result<Vec<T>, SQLError> where F : FnMut(Result<Row, Error>) -> T {
        let db = DataBase::from_env();
        match db {
            Ok(db) => {
                let tmp: Vec<T> = match db.execute(&self.into(), row_map){
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
impl From<QML> for SQL{
    fn from(qml: QML) -> Self {
        let sql = SQL::new(&qml)
            .unwrap_or_else(|_| panic!("\"{}\" is an invalid qml state", *qml));

        match sql {
            SQL::Insert(_) |
            SQL::Update(_) |
            SQL::Delete(_) => sql,

            _=> panic!("\"{}\" is an invalid qml state", *qml)
        }
    }
}
impl From<&QML> for SQL{
    fn from(qml: &QML) -> Self {
        let sql = SQL::new(&qml)
            .unwrap_or_else(|_| panic!("\"{}\" is an invalid qml state", **qml));

        match sql {
            SQL::Insert(_) |
            SQL::Update(_) |
            SQL::Delete(_) => sql,

            _=> panic!("\"{}\" is an invalid qml state", **qml)
        }
    }
}
impl From<&QML> for SQLLanguage{
    fn from(_: &QML) -> Self {
        SQLLanguage::QML
    }
}
/// Represents a data control language (DCL) SQL command.
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
    fn execute<T, F>(&self, row_map: F) -> Result<Vec<T>, SQLError> where F : FnMut(Result<Row, Error>) -> T {
        let db = DataBase::from_env();
        match db {
            Ok(db) => {
                let tmp: Vec<T> = match db.execute(&self.into(), row_map){
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
impl From<DCL> for SQL{
    fn from(dcl: DCL) -> Self {
        let sql = SQL::new(&dcl)
            .unwrap_or_else(|_| panic!("\"{}\" is an invalid dcl state", *dcl));

        match sql {
            SQL::Grant(_) |
            SQL::Revoke(_) => sql,

            _=> panic!("\"{}\" is an invalid dcl state", *dcl)
        }
    }
}
impl From<&DCL> for SQL{
    fn from(dcl: &DCL) -> Self {
        let sql = SQL::new(&dcl)
            .unwrap_or_else(|_| panic!("\"{}\" is an invalid dcl state", **dcl));

        match sql {
            SQL::Grant(_) |
            SQL::Revoke(_) => sql,

            _=> panic!("\"{}\" is an invalid dcl state", **dcl)
        }
    }
}
impl From<&DCL> for SQLLanguage{
    fn from(_: &DCL) -> Self {
        SQLLanguage::DCL
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

enum ParseMode{
    Regular,
    Comment,
    String(char),
}

impl ParseMode {
    pub fn parse(self, buffer: &mut Vec<char>, cmds: &mut Vec<SQL>, ch: char) -> Self {
        match self{
            ParseMode::Regular => self.regular_parse(buffer, cmds, ch),
            ParseMode::Comment => self.comment_parse(ch),
            ParseMode::String(_) => self.string_parse(buffer, ch),
        }
    }

    fn regular_parse(self, buffer: &mut Vec<char>, cmds: &mut Vec<SQL>, ch: char) -> Self{
        match ch {
            //new string
            '\"' |
            '\'' |
            '`'  => {
                buffer.push(ch);
                ParseMode::String(ch)
            },
            //comment check
            '-' => {
                match buffer.last() {
                    Some('-') => {
                        buffer.pop();
                        ParseMode::Comment
                    },
                    None | Some(_) => {
                        buffer.push(ch);
                        self
                    }
                }
            },
            //end of line
            ';' => {
                match SQL::new(&buffer.drain(..).collect::<String>()) {
                    Ok(val) => cmds.push(val),
                    Err(err) => log::error!("Error - {:?}", err),
                }
                self
            },
            //new line
            '\r' |
            '\t' |
            '\n' |
            ' ' => {
                match buffer.last() {
                    Some(' ') |
                    None => self,
                    Some(_) => {
                        buffer.push(' ');
                        self
                    },
                }
            }
            //regular char
            _=> {
                buffer.push(ch);
                self
            }
        }
    }

    fn comment_parse(self, ch: char) -> Self{
        match ch {
            '\n' => ParseMode::Regular,
            _=> self
        }
    }

    fn string_parse(self, buffer: &mut Vec<char>, ch: char) -> Self {
        match self {
            ParseMode::String(end_cond) => {
                buffer.push(ch);

                if ch == end_cond {
                    return ParseMode::Regular
                }
                self
            },
            _=> {
                panic!()
            }
        }
    }
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
    /// Parses an SQL command from a string.
    ///
    /// # Arguments
    ///
    /// * `query` - a string slice containing the SQL command to parse.
    ///
    /// /// # Example
    ///
    /// ```
    /// use sql::SQL;
    ///
    /// let query = "SELECT * FROM users WHERE age > 25";
    /// let sql = SQL::new(query).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// This function returns a `SQLError` variant if the command is invalid or if there is an error
    /// inserting the contents of a file specified in the command (using the `@` syntax).
    pub fn new(query: &str) -> Result<SQL, SQLError> {
        
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

    /// Returns a borrow `DDL` variant of the `SQL` enum if it exists, otherwise returns `None`.
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
    /// Returns a mutable borrow of `DDL` variant of the `SQL` enum if it exists otherwise returns `None`.
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

    /// Returns a borrow `QDL` variant of the `SQL` enum if it exists, otherwise returns `None`.
    pub fn qdl(&self) -> Option<&QDL> {
        match self {
            SQL::Select(qdl) => Some(qdl),
            _ => None,
        }
    }
    /// Returns a mutable borrow of `QDL` variant of the `SQL` enum if it exists otherwise returns `None`.
    pub fn qdl_mut(&mut self) -> Option<&mut QDL> {
        match self {
            SQL::Select(qdl) => Some(qdl),
            _ => None,
        }
    }
    
    /// Returns a borrow `QML` variant of the `SQL` enum if it exists, otherwise returns `None`.
    pub fn qml(&self) -> Option<&QML> {
        match self{
            SQL::Insert(cmd) |
            SQL::Update(cmd) |
            SQL::Delete(cmd) => Some(cmd),
            _ => None
        }
    }
    /// Returns a mutable borrow of `QML` variant of the `SQL` enum if it exists, otherwise returns `None`.
    pub fn qml_mut(&mut self) -> Option<&mut QML> {
        match self{
            SQL::Insert(cmd) |
            SQL::Update(cmd) |
            SQL::Delete(cmd) => Some(cmd),
            _ => None
        }
    }

    /// Returns a borrow `DCL` variant of the `SQL` enum if it exists, otherwise returns `None`.
    pub fn dcl(&self) -> Option<&DCL> {
        match self {
            SQL::Grant(cmd) |
            SQL::Revoke(cmd) => Some(cmd),
            _ => None,
        }
    }
    /// Returns a mutable borrow of `DCL` variant of the `SQL` enum if it exists, otherwise returns `None`.
    pub fn dcl_mut(&mut self) -> Option<&mut DCL> {
        match self {
            SQL::Grant(cmd) |
            SQL::Revoke(cmd) => Some(cmd),
            _ => None,
        }
    }

    /// Returns a enum of SQL language type
    pub fn get_language(&self) -> SQLLanguage {
        match &self {
            SQL::Create(cmd) |
            SQL::Alter(cmd) |
            SQL::Drop(cmd) | 
            SQL::Truncate(cmd) |
            SQL::Show(cmd) => cmd.into(),

            SQL::Select(cmd) => cmd.into(),

            SQL::Insert(cmd) |
            SQL::Update(cmd) |
            SQL::Delete(cmd)  => cmd.into(),

            SQL::Grant(cmd)  |
            SQL::Revoke(cmd) => cmd.into(),
        }
    }


    /// Returns vector of SQL from a file.
    /// 
    /// # Arguments
    ///
    /// * `file_path` - a string slice of file path.
    /// 
    /// # Example
    ///
    /// ```rust
    /// let file_path = "file_parsing_multiple_cmds.sql";
    /// 
    /// let content = "                
    /// --organizational tags\n
    /// INSERT INTO tag (colour, symbol, tag_type) VALUES (\"ffffff\", \"Web\", 2);
    /// INSERT INTO tag (colour, symbol, tag_type) VALUES (\"5CFFA1\", \"Shader\", 2);
    /// 
    /// 
    /// INSERT INTO tag (colour, symbol, tag_type) VALUES (\"FF5CBA\", \"GameJam\", 2);
    /// "
    /// 
    /// let mut file = File::create(&file_name).unwrap();
    /// file.write(&content.as_bytes()).unwrap();
    /// 
    /// let actual = SQL::from_file(file_path);
    /// let expected = vec![
    ///     SQL::new("INSERT INTO tag (colour, symbol, tag_type) VALUES (\"ffffff\", \"Web\", 2)").unwrap(),
    ///     SQL::new("INSERT INTO tag (colour, symbol, tag_type) VALUES (\"5CFFA1\", \"Shader\", 2)").unwrap(),
    ///     SQL::new("INSERT INTO tag (colour, symbol, tag_type) VALUES (\"FF5CBA\", \"GameJam\", 2)").unwrap(),
    /// ];
    /// 
    /// assert_eq!(
    ///     actual.unwrap(),
    ///     expected
    /// );
    /// ```
    /// 
    /// # Errors
    /// 
    /// This function returns a `std::io::Error` if the file cannot be loaded
    pub fn from_file(file_path: &str) -> Result<Vec<SQL>, std::io::Error> {
        let mut file: File = File::open(file_path)?;

        const BUFFER_SIZE: usize = 100;

        let mut buffer = [0; BUFFER_SIZE];
        let mut parse_mode = ParseMode::Regular;

        let mut cmd : Vec<char> = Vec::new();

        let mut results: Vec<SQL> = Vec::new();
        
        loop {
            match file.read(&mut buffer) {
                Ok(0) => {
                    match SQL::new(&cmd.clone().into_iter().collect::<String>()) {
                        Ok(val) => results.push(val),
                        Err(err) => log::error!("Error - {:?}", err),
                    }
                    break;
                },
                Ok(n) => {
                    for ch in buffer[..n].iter() {
                        parse_mode = parse_mode.parse(&mut cmd, &mut results, *ch as char);
                    }
                }
                Err(err) => {
                    return Err(err)
                }
            }
        }
        Ok(results)
    }

    /// Saves a vector of SQL commands into a file
    /// 
    /// # Arguments
    /// 
    /// * `file_path` - a string slice of a file path
    /// * `queries` - a vector of `SQL`
    /// 
    /// # Errors
    /// 
    /// This function return `std::io::Error` if the function fails to create a file with given parameters
    pub fn save_to_file(file_path: &str, queries: &Vec<SQL>) -> Result<(), std::io::Error> {
        let mut file: File = File::create(file_path)?;

        let mut content = String::new();

        queries.iter()
            .for_each(|query| content.push_str(&format!("{};\n", query.to_string())));

        file.write(content.as_bytes())?;

        Ok(())
    }
}

impl DatabaseExecute for SQL{
    type RowError = SQLError;
    fn execute<T, F>(&self, row_map: F) -> Result<Vec<T>, SQLError> where F : FnMut(Result<Row, Error>) -> T {
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


#[cfg(test)]
mod tests{
    #[allow(unused_imports)]
    use crate::test_tools::file_env::FileEnv;

    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_imports)]
    use indoc::formatdoc;
    #[allow(unused_imports)]
    use indoc::indoc;

    //extracting commands from file
    #[test]
    fn file_parsing_multi_line_cmd() {
        let file_name = "file_parsing_multi_line_cmd.sql";
        let _file = FileEnv::new(
            file_name,
            "
                CREATE TABLE tag ( --multi line cmd
                    id INT AUTO_INCREMENT,
                    colour CHAR(6),
                    symbol VARCHAR(50) NOT NULL UNIQUE,
                    tag_type INT NOT NULL,
                    PRIMARY KEY(id)
                );
                
                
                SELECT \"\n\t\t\n\t\t\";--multi line but in a string
            "
        );

        let actual = SQL::from_file(file_name);
        let expected = vec![
            SQL::new("CREATE TABLE tag ( id INT AUTO_INCREMENT, colour CHAR(6), symbol VARCHAR(50) NOT NULL UNIQUE, tag_type INT NOT NULL, PRIMARY KEY(id) )").unwrap(),
            SQL::new("SELECT \"\n\t\t\n\t\t\"").unwrap(),
        ];
        assert_eq!(
            actual.unwrap(),
            expected
        );
    }

    #[test]
    fn file_parsing_multiple_cmds() {

        let file_path = "file_parsing_multiple_cmds.sql";
        let _file = FileEnv::new(
            file_path,
            "                
            --organizational tags\n
            INSERT INTO tag (colour, symbol, tag_type) VALUES (\"ffffff\", \"Web\", 2);
            INSERT INTO tag (colour, symbol, tag_type) VALUES (\"5CFFA1\", \"Shader\", 2);
            
            
            INSERT INTO tag (colour, symbol, tag_type) VALUES (\"FF5CBA\", \"GameJam\", 2);

            "
        );

        let actual = SQL::from_file(file_path);
        let expected = vec![
            SQL::new("INSERT INTO tag (colour, symbol, tag_type) VALUES (\"ffffff\", \"Web\", 2)").unwrap(),
            SQL::new("INSERT INTO tag (colour, symbol, tag_type) VALUES (\"5CFFA1\", \"Shader\", 2)").unwrap(),
            SQL::new("INSERT INTO tag (colour, symbol, tag_type) VALUES (\"FF5CBA\", \"GameJam\", 2)").unwrap(),
        ];
        assert_eq!(
            actual.unwrap(),
            expected
        );
    }

    #[test]
    fn file_insertion_1() {
        let file_name_1 = "file_insertion_1_1.sql";
        let file_name_2 = "file_insertion_1_2.txt";

        let _file_1 = FileEnv::new(
            file_name_1,
            &format!(
                "INSERT INTO tag (colour, symbol, tag_type) VALUES (\"#file:({} as S)\", \"Web\", 2);",
                file_name_2
            )
        );
        let _file_2 = FileEnv::new(
            file_name_2,
            "ffffff"
        );
        
        let expected = vec![
            SQL::new("INSERT INTO tag (colour, symbol, tag_type) VALUES (\"ffffff\", \"Web\", 2)").unwrap(),
        ];
        
        let actual = SQL::from_file(file_name_1);

        assert_eq!(
            actual.unwrap(),
            expected
        );

        let actual = SQL::new(&format!("#file:({} as S)", file_name_1));
        let expected = SQL::new("INSERT INTO tag (colour, symbol, tag_type) VALUES (\"ffffff\", \"Web\", 2);").unwrap();
        assert_eq!(
            actual.unwrap(),
            expected
        );
    }

    #[test]
    fn file_insertion_2() {
        let file_name_1 = "file_insertion_2_1.sql";
        let file_name_2 = "content_1.md";
        let file_name_3 = "content_2.md";

        let _file_1 = FileEnv::new(
            file_name_1,
            &formatdoc! {
                "
                --create project & dev log
                CREATE TABLE project (proj_tag INT, repo TINYTEXT NOT NULL, first_push DATE NOT NULL, last_push DATE, Foreign KEY(proj_tag) REFERENCES tag(id));
                CREATE TABLE dev_log (tag_id INT, created TIMESTAMP, body TEXT NOT NULL, Foreign KEY(tag_id) REFERENCES tag(id));
                
                --update symbol => tag_name & tag_name can be null and doesn't have to be unique
                ALTER TABLE tag ADD COLUMN tag_name VARCHAR(100);
                UPDATE tag SET tag_name=tag.symbol;
                ALTER TABLE tag DROP COLUMN symbol;
                
                INSERT INTO project (proj_tag, repo, first_push) VALUES (30, \"link\", '2022-2-01');
                INSERT INTO tag (colour, tag_name, tag_type) VALUES (\"ffffff\", \"proj_name\", 3);
                INSERT INTO dev_log (tag_id, body) VALUES ((SELECT COUNT(*) FROM tag LIMIT 1), \"#file:({} as S)\");
                
                INSERT INTO project (proj_tag, repo, first_push) VALUES (30, \"link\", '2022-2-01');
                INSERT INTO tag (colour, tag_name, tag_type) VALUES (\"ffffff\", \"proj_name\", 3);
                INSERT INTO dev_log (tag_id, body) VALUES ((SELECT COUNT(*) FROM tag LIMIT 1), \"#file:({} as S)\");
                ",
                file_name_2,
                file_name_3
            }
        );

        let _file_2 = FileEnv::new(
            file_name_2,
            &format!("One line of content")
        );
        let _file_3 = FileEnv::new(
            file_name_3,
            &format!("There are two lines of content.\nI am the second line.")
        );

        let expected = vec![
            SQL::new("CREATE TABLE project (proj_tag INT, repo TINYTEXT NOT NULL, first_push DATE NOT NULL, last_push DATE, Foreign KEY(proj_tag) REFERENCES tag(id))").unwrap(),
            SQL::new("CREATE TABLE dev_log (tag_id INT, created TIMESTAMP, body TEXT NOT NULL, Foreign KEY(tag_id) REFERENCES tag(id))").unwrap(),
            SQL::new("ALTER TABLE tag ADD COLUMN tag_name VARCHAR(100)").unwrap(),
            SQL::new("UPDATE tag SET tag_name=tag.symbol").unwrap(),
            SQL::new("ALTER TABLE tag DROP COLUMN symbol").unwrap(),
            SQL::new("INSERT INTO project (proj_tag, repo, first_push) VALUES (30, \"link\", '2022-2-01')").unwrap(),
            SQL::new("INSERT INTO tag (colour, tag_name, tag_type) VALUES (\"ffffff\", \"proj_name\", 3)").unwrap(),
            SQL::new(&format!("INSERT INTO dev_log (tag_id, body) VALUES ((SELECT COUNT(*) FROM tag LIMIT 1), \"{}\")", "One line of content")).unwrap(),
            SQL::new("INSERT INTO project (proj_tag, repo, first_push) VALUES (30, \"link\", '2022-2-01')").unwrap(),
            SQL::new("INSERT INTO tag (colour, tag_name, tag_type) VALUES (\"ffffff\", \"proj_name\", 3)").unwrap(),
            SQL::new(&format!("INSERT INTO dev_log (tag_id, body) VALUES ((SELECT COUNT(*) FROM tag LIMIT 1), \"{}\")", "There are two lines of content.\nI am the second line.")).unwrap(),
        ];
        
        let actual = SQL::from_file(file_name_1).unwrap();

        actual.iter()
            .zip(expected.iter())
            .for_each(|(actual, expected)| {
                assert_eq!(
                    actual,
                    expected
                );
            })
    }


    //testing SQL parsing
    //Data Definition Language
    #[test]
    fn create_test_1() {
        let input = "CREATE TABLE MyTable (RowId INT64 NOT NULL, `Order` INT64 ) PRIMARY KEY (RowId)";

        let actual = SQL::new(input).unwrap();

        
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

        let actual = SQL::new(input).unwrap();

        
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

        let actual = SQL::new(input).unwrap();

        
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

        let actual = SQL::new(input).unwrap();

        
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

        let actual = SQL::new(input).unwrap();

        
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

        let actual = SQL::new(input).unwrap();

        
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

        let actual = SQL::new(input).unwrap();

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

        let actual = SQL::new(input).unwrap();

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

        let actual = SQL::new(input).unwrap();

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

        let actual = SQL::new(input).unwrap();

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