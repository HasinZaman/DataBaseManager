use std::{fmt, env::VarError};

use mysql::{Error, Row};
use regex::Regex;
use lazy_static::lazy_static;

use super::data_base::{DataBase};

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
        lazy_static! {
            static ref SELECT_REGEX: Regex = Regex::new("^[Ss][Ee][Ll][Ee][Cc][Tt] .+").unwrap();
        };
        lazy_static! {
            static ref INSERT_REGEX: Regex = Regex::new("^[Ii][Nn][Ss][Ee][Rr][Tt] .+").unwrap();
        };
        lazy_static! {
            static ref UPDATE_REGEX: Regex = Regex::new("^[Uu][Pp][Dd][Aa][Tt][Ee] .+").unwrap();
        };
        lazy_static! {
            static ref DELETE_REGEX: Regex = Regex::new("^[Dd][Ee][Ll][Ee][Tt][Ee] .+").unwrap();
        };

        if let Some(_mat) = SELECT_REGEX.find(&query){
            return Ok(Query::Select(query.to_string()));
        }
        else if let Some(_mat) = INSERT_REGEX.find(&query){
            return Ok(Query::Insert(query.to_string()));
        }
        else if let Some(_mat) = UPDATE_REGEX.find(&query){
            return Ok(Query::Update(query.to_string()));
        }
        else if let Some(_mat) = DELETE_REGEX.find(&query){
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