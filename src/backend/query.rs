use std::fmt;

use regex::Regex;

const SELECT_REGEX : &str = "S|sE|eL|lE|eC|cT|t .+";
const INSERT_REGEX : &str = "I|iN|nS|sE|eR|rT|t .+";
const UPDATE_REGEX : &str = "U|uP|pD|dA|aT|tE|e .+";
const DELETE_REGEX : &str = "D|dE|eL|lE|eT|tE|e .+";

#[derive(Debug)]
pub enum QueryErr{
    NoMethodInQuery,
    InvalidQuery{expected_variant: Query},
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
    pub fn from(query: &str) -> Result<Query, QueryErr> {
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

        Result::Err(QueryErr::NoMethodInQuery)
    }

    pub fn execute(&self) {
        //execute thing and return rows
        todo!();
    }
}


impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Query::Select(query) | Query::Insert(query) | Query::Update(query) | Query::Delete(query) => write!(f, "{}", query)
        }
    }
}