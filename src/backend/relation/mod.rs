use mysql::Error;

use crate::backend::data_base::DataBase;

use self::{table::Table, view::View};

pub mod table;
pub mod view;

//Relation Defines the two relations that can exist in a MySql database
#[derive(Clone, Debug)]
pub enum Relation{
    Table(Table),
    View(View),
}

impl Relation {
    pub fn get_relations() -> Result<Vec<Relation>, Error> {
        match DataBase::from_env() {
            Ok(db) => {
                let rows: Vec<Relation> = db.execute(
                    "SHOW FULL TABLES",
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
    /// name method allows for easy access to name of relation from table or query
    pub fn name(&self) -> String {
        match self {
            Relation::Table(table) => table.name.clone(),
            Relation::View(view) => view.name.clone()
        }
    }
}