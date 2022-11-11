use crate::backend::query::{Query};

#[derive(Clone, Debug)]
pub struct View{
    name: String,
    query: Query
}

impl View {
    pub fn new(name: String, query: Query) -> Option<View> {
        if let Query::Select(_query) = &query {
            return Some(
                View{
                    name: name,
                    query: query
                }
            );
        }
        None
    }
}