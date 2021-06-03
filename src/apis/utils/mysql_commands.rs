pub mod mysql_commands {

    use mysql::*;
    use mysql::prelude::*;

    pub fn create_table(s: String) -> Result<()> {
        let url: &str = "mysql://root:@localhost:3306/mws_database";

        let pool: Pool = Pool::new(url)?;

        let mut conn = pool.get_conn()?;

        conn.query_drop(s.to_string())
    }

    pub fn check_table_exists(table_name: String) -> Result<bool> {

        let url: &str = "mysql://root:@localhost:3306/mws_database";

        let pool: Pool = Pool::new(url).unwrap();

        let mut conn = pool.get_conn().unwrap();

        let if_exist = format!("SELECT COUNT(*)
        FROM information_schema.tables 
        WHERE table_schema = DATABASE()
        AND table_name = '{}';", table_name);

        let table_exists = conn.query_map(
            if_exist,
            |count: usize| {
                count
            }
        ).unwrap();

        match & table_exists[0] {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(mysql::Error::MySqlError(MySqlError {  message: "Error in fetching the table status".to_string(), state: "MySqlError".to_string(), code: 1004 }))
        }
    }
}