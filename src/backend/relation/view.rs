use regex::Regex;

use crate::backend::{query::{Query}, data_base::DataBase};

#[derive(Clone, Debug)]
pub struct View{
    pub name: String,
    pub query: Query
}

impl View {
    pub fn from_db(name: &str) -> Option<View> {
        let db = DataBase::from_env().unwrap();

        let tmp : Vec<String> = db.execute(
            &format!(r"SHOW CREATE TABLE {}", name),
            |row| {
                let tmp_str: String = row.unwrap().get(1).unwrap();

                let view_def : Regex = Regex::new("[s|S][e|E][l|L][e|E][c|C][t|T] [a-zA-Z0-9`., ()=]+").unwrap();

                view_def.captures(&tmp_str)
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str()
                    .to_string()
            }
        ).unwrap();

        match tmp.get(0) {
            Some(query) => {
                Some(
                    View {
                        name: name.to_string(),
                        query: match Query::from(query) {
                            Ok(query) => query,
                            Err(_err) => {
                                return None
                            }
                        }
                    }
                )
            },
            None => None
        }

    }

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