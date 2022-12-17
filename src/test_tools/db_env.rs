use crate::backend::{sql::SQL, data_base::DataBase};

/// Struct representing a database environment
pub struct DbEnv{
    /// A list of SQL commands to be executed when the environment is reset
    undo: Vec<SQL>
}
impl DbEnv{
    /// Creates a new DbEnv with the given setup and undo commands
    ///
    /// # Arguments
    ///
    /// * `set_up` - A list of SQL commands to be executed when the environment is set up
    /// * `undo` - A list of SQL commands to be executed when the environment is dropped from memory
    pub fn new(set_up: Vec<SQL>, undo: Vec<SQL>) -> DbEnv {
        let tmp = DbEnv { undo: undo };
        
        let db = DataBase::from_env().unwrap();

        db.execute_multiple(&set_up).unwrap();

        tmp
    }
}
impl Drop for DbEnv{
    /// Resets the database environment by executing the undo commands
    fn drop(&mut self) {
        let db = DataBase::from_env().unwrap();

        db.execute_multiple(&self.undo).unwrap();
    }
}