pub mod parser {

    use mysql::*;
    use mysql::prelude::*;

    use crate::apis::utils::mysql_commands::mysql_commands::check_table_exists;
    
    pub fn row_parser(s: String, table: String, n: usize) -> Vec<Vec<String>> {
        let url: &str = "mysql://root:@localhost:3306/mws_database";
    
        let pool: Pool = Pool::new(url).unwrap();

        let conn = pool.get_conn().unwrap();

        let mut outer_v: Vec<Vec<String>> = Vec::new();

        match check_table_exists(table.to_string()) {
            Ok(true) => {
                let rows: Vec<Row> = s.fetch(conn).unwrap();

                for row in rows {

                    let mut v: Vec<String> = Vec::new();
                    
                    for i in 0..n {

                        let value = row.get_opt::<String, usize>(i);

                        match value {
                            Some(Ok(val)) => v.push(val.to_string()),
                            Some(Err(_)) => (),
                            None => ()
                        }

                    }
                    outer_v.push(v.clone());
                }
            },
            Ok(false) => (),
            Err(_) => ()
        }
        outer_v
    }
}