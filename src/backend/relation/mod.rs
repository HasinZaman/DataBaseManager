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
    /// name method allows for easy access to name of relation from table or query
    pub fn name(&self) -> String {
        match self {
            Relation::Table(table) => table.name.clone(),
            Relation::View(view) => view.name.clone()
        }
    }
}