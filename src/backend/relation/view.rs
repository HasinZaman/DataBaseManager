use regex::Regex;
use lazy_static::lazy_static;

use crate::backend::{ sql::{QDL, SQL, DDL}, data_base::DataBase};

use super::RelationMethods;

#[derive(Clone, Debug)]
pub struct View{
    pub name: String,
    pub query: QDL
}

impl View {
    pub fn from_db(name: &str) -> Option<View> {
        let db = DataBase::from_env().unwrap();

        let tmp : Vec<String> = db.execute(
            &SQL::new(&format!(r"SHOW CREATE TABLE {}", name)).unwrap(),
            |row| {
                let tmp_str: String = row.unwrap().get(1).unwrap();

                lazy_static! {
                    static ref VIEW_REGEX : Regex = Regex::new("[sS][eE][lL][eE][cC][tT] [a-zA-Z0-9`., ()=]+").unwrap();
                };

                VIEW_REGEX.captures(&tmp_str)
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
                        query: SQL::new(query).unwrap().qdl().unwrap().clone()
                    }
                )
            },
            None => None
        }

    }

    pub fn new(name: &str, query: QDL) -> View {
        View{
            name: name.to_string(),
            query: query
        }
    }
}

impl RelationMethods for View{
    fn select(&self) -> QDL {
        QDL(format!("SELECT * FROM {}", self.name))
    }
    fn drop(&self) -> DDL{
        DDL(format!("DROP VIEW {}", self.name))
    }

    fn create(&self) -> DDL {
        SQL::new(&format!("CREATE VIEW {} AS {}", self.name, *self.query)).unwrap().ddl().unwrap().clone()
    }
}