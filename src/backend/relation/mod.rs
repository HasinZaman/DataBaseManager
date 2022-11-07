use self::table::Table;

use super::query::Query;

pub mod table;

//Relation Defines the two relations that can exist in a MySql database
#[derive(Clone, Debug)]
pub enum Relation{
    Table(Table),
    View(Query),
}