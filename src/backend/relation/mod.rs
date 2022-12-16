use mysql::Error;

use crate::backend::data_base::DataBase;

use self::{table::Table, view::View};

use super::sql::{SQL, DDL, QDL};

pub mod table;
pub mod view;
pub mod paths;

/// A trait representing methods for generating SQL statements for relations.
pub trait RelationMethods {
    /// Returns a `QDL` representing a `SELECT` statement for the relation.
    fn select(&self) -> QDL;
    /// Returns a `DDL` representing a `DROP` statement for the relation.
    fn drop(&self) -> DDL;
    /// Returns a `DDL` representing a `CREATE` statement for the relation.
    fn create(&self) -> DDL;
}

/// An enumeration representing a relation.
///
/// A relation can either be a `Table` or a `View`.
#[derive(Clone, Debug)]
pub enum Relation{
    Table(Table),
    View(View),
}

impl Relation {
    /// Returns a vector of `Relation`s from the database.
    ///
    /// # Errors
    ///
    /// This function will return an error if there is a problem accessing the database or executing the required queries.
    pub fn get_relations() -> Result<Vec<Relation>, Error> {
        match DataBase::from_env() {
            Ok(db) => {
                let rows: Vec<Relation> = db.execute(
                    &SQL::new("SHOW FULL TABLES").unwrap(),
                    |row| {
                        match row {
                            Ok(row) => {
                                let name: String = row.get(0).unwrap();
                                let relation_type: String = row.get(1).unwrap();

                                return Some((name, relation_type));
                            },
                            Err(_err) => {
                                return None;
                            }
                        };
                    }
                ).unwrap()
                .iter()
                .filter(|val| {
                    match val {
                        Some(_str) => true,
                        None => false,
                    }
                })
                .map(|val| {
                    match val {
                        Some(name_type_pair) => name_type_pair.clone(),
                        None => panic!(),
                    }
                })
                .map(|row| {
                    let row : (&str, &str) = (&row.0, &row.1);
                    match row {
                        (name, "BASE TABLE") => {
                            Relation::Table(Table::from_db(name).unwrap())
                        },
                        (name, "VIEW") => {
                            Relation::View(View::from_db(name).unwrap())
                        },
                        _ => {
                            panic!("Invalid row")
                        }
                    }
                })
                .collect();

                return Ok(rows)
            },
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }
    
    /// Returns the name of the relation as a `String`.
    pub fn name(&self) -> String {
        match self {
            Relation::Table(table) => table.name.clone(),
            Relation::View(view) => view.name.clone()
        }
    }
}

impl RelationMethods for Relation{
    fn select(&self) -> QDL {
        match self{
            Relation::Table(table) => table.select(),
            Relation::View(view) => view.select(),
        }
    }
    fn drop(&self) -> DDL {
        match self {
            Relation::Table(table) => table.drop(),
            Relation::View(view) => view.drop(),
        }
    }
    fn create(&self) -> DDL {
        match self {
            Relation::Table(table) => table.create(),
            Relation::View(view) => view.create(),
        }
    }
}