use crate::backend::{sql::SQL, data_base::DataBase};

pub struct DbEnv{
    undo: Vec<SQL>
}
impl DbEnv{
    pub fn new(set_up: Vec<SQL>, undo: Vec<SQL>) -> DbEnv {
        let tmp = DbEnv { undo: undo };
        
        let db = DataBase::from_env().unwrap();

        db.execute_multiple(&set_up).unwrap();

        tmp
    }
}
impl Drop for DbEnv{
    fn drop(&mut self) {
        let db = DataBase::from_env().unwrap();

        db.execute_multiple(&self.undo).unwrap();
    }
}