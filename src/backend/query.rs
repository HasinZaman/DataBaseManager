#[derive(Clone)]
pub enum Query {
    Select(String),
    Insert(String),
    Update(String),
    Delete(String)
}
