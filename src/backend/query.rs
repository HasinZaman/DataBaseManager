/// Query enum defines structure of a SQL Query
#[derive(Clone, Debug)]
pub enum Query {
    Select(String),
    Insert(String),
    Update(String),
    Delete(String)
}
