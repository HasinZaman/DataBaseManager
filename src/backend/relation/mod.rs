use self::table::Table;

use super::query::Query;

pub mod table;

#[derive(Clone)]
pub enum Relation{
    Table(Table),
    View(Query),
}