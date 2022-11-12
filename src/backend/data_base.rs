use mysql::{prelude::*, Opts, Conn, Row, Error};

#[derive(Debug)]
pub struct DataBase {
    pub db_host: String,
    pub db_port: String,
    pub db_name: String,
    pub db_username: String,
    pub db_password: String,
}

impl DataBase {
    fn get_conn(&self) -> mysql::Conn {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            self.db_username, self.db_password, self.db_host, self.db_port, self.db_name
        );

        println!("{}", url);

        let url: Opts = Opts::from_url(&url).unwrap();

        Conn::new(url).unwrap()
    }

    

    pub fn execute<E>(&self, command: &str, row_map: fn(row: Result<Row, Error>) -> E ) -> Result<Vec<E>, Error> {
        let mut conn = self.get_conn();

        let statement = conn.prep(command).unwrap();

        let rows: Vec<E>;
        match conn.exec_iter(&statement, ()){
            Ok(iter) => {
                rows = iter.map(row_map).collect();
            },
            Err(err) => return Err(err),
        }

        let _result = conn.close(statement);
        
        Ok(rows)
    }
}
