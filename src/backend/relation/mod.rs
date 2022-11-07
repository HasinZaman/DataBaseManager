use self::table::Table;

use super::query::Query;

pub mod table;

//Relation Defines the two relations that can exist in a MySql database
#[derive(Clone, Debug)]
pub enum Relation{
    Table(Table),
    View(Query),
}

impl Relation {
    /// name method allows for easy access to name of relation from table or query
    pub fn name(&self) -> String {
        match self {
            Relation::Table(table) => table.name.clone(),
            Relation::View(_view) => todo!()
        }
    }
}