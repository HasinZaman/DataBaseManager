use std::{fmt, env::VarError};

use mysql::{Error, Row};
use regex::Regex;

use super::data_base::{DataBase};

const SELECT_REGEX : &str = "^[Ss][Ee][Ll][Ee][Cc][Tt] .+";
const INSERT_REGEX : &str = "^[Ii][Nn][Ss][Ee][Rr][Tt] .+";
const UPDATE_REGEX : &str = "^[Uu][Pp][Dd][Aa][Tt][Ee] .+";
const DELETE_REGEX : &str = "^[Dd][Ee][Ll][Ee][Tt][Ee] .+";

#[derive(Debug)]
pub enum QueryError{
    NoMethodInQuery,
    InvalidQuery{expected_variant: Query},
    FailedToConnect(VarError),
    Execution(Error),
    Err(String)
}

/// Query enum defines structure of a SQL Query
#[derive(Clone, Debug)]
pub enum Query {
    Select(String),
    Insert(String),
    Update(String),
    Delete(String)
}

impl Query {
    pub fn from(query: &str) -> Result<Query, QueryError> {
        if let Some(_mat) = Regex::new(SELECT_REGEX).unwrap().find(&query){
            return Ok(Query::Select(query.to_string()));
        }
        else if let Some(_mat) = Regex::new(INSERT_REGEX).unwrap().find(&query){
            return Ok(Query::Insert(query.to_string()));
        }
        else if let Some(_mat) = Regex::new(UPDATE_REGEX).unwrap().find(&query){
            return Ok(Query::Update(query.to_string()));
        }
        else if let Some(_mat) = Regex::new(DELETE_REGEX).unwrap().find(&query){
            return Ok(Query::Delete(query.to_string()));
        }

        Result::Err(QueryError::NoMethodInQuery)
    }

    pub fn execute<E, F>(&self, row_map: F) -> Result<Vec<E>, QueryError> where F : Fn(Result<Row, Error>) -> E {
        let db = DataBase::from_env();

        match db {
            Ok(db) => {
                let tmp: Vec<E> = match db.execute(&self.to_string(), row_map){
                    Ok(val) => val,
                    Err(err) => return Err(QueryError::Execution(err)),
                };

                Ok(tmp)
            },
            Err(err) => {
                Err(QueryError::FailedToConnect(err))
            }
        }
    }
}


impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Query::Select(query) | Query::Insert(query) | Query::Update(query) | Query::Delete(query) => write!(f, "{}", query)
        }
    }
}