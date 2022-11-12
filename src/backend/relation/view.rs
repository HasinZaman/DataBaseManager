use crate::backend::query::{Query};

#[derive(Clone, Debug)]
pub struct View{
    pub name: String,
    pub query: Query
}

impl View {
    pub fn new(name: &str, query: Query) -> Option<View> {
        if let Query::Select(_query) = &query {
            return Some(
                View{
                    name: name.to_string(),
                    query: query
                }
            );
        }
        None
    }
}