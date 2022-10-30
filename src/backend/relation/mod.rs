use self::table::Table;

use super::query::Query;

pub mod table;

pub enum Relation{
    Table(Table),
    View(Query),
}