use regex::Regex;
use lazy_static::lazy_static;

use crate::backend::{ sql::{QDL, SQL, DDL}, data_base::DataBase};

use super::RelationMethods;

#[derive(Clone, Debug)]
pub struct View{
    /// The name of the view.
    pub name: String,
    /// The query defining the view.
    pub query: QDL
}

impl View {
    /// Returns a `View` created from a database with the given name.
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

    /// Returns a new `View` with the given name and query.
    pub fn new(name: &str, query: QDL) -> View {
        View{
            name: name.to_string(),
            query: query
        }
    }
}

impl RelationMethods for View{
    /// Returns a `QDL` representing a `SELECT` statement for the view.
    fn select(&self) -> QDL {
        QDL(format!("SELECT * FROM {}", self.name))
    }
    /// Returns a `DDL` representing a `DROP` statement for the view.
    fn drop(&self) -> DDL{
        DDL(format!("DROP VIEW {}", self.name))
    }
    /// Returns a `DDL` representing a `CREATE` statement for the view.
    fn create(&self) -> DDL {
        SQL::new(&format!("CREATE VIEW {} AS {}", self.name, *self.query)).unwrap().ddl().unwrap().clone()
    }
}