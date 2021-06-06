pub mod parser {

    use mysql::*;
    use mysql::prelude::*;
    
    pub fn row_parser(s: String, n: usize) -> Vec<Vec<String>> {
        let url: &str = "mysql://root:@localhost:3306/mws_database";
    
        let pool: Pool = Pool::new(url).unwrap();

        let conn = pool.get_conn().unwrap();

        let mut outer_v: Vec<Vec<String>> = Vec::new();

        let rows: Vec<Row> = s.fetch(conn).unwrap();

        for row in rows {

            println!("ROW: {:?}", &row);

            let mut v: Vec<String> = Vec::new();
            
            for i in 0..n {

                let value = row.get_opt::<String, usize>(i);

                println!("{:?}", &value);

                match value {
                    Some(Ok(val)) => v.push(val.to_string()),
                    Some(Err(_)) => v.push("0".to_string()),
                    None => v.push("0".to_string())
                }
            }
            outer_v.push(v.clone());
        }
        outer_v
    }
}