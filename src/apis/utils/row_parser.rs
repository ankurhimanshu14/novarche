pub mod parser {

    use mysql::*;
    use mysql::prelude::*;
    
    pub fn row_parser(s: String, n: usize) -> Vec<Vec<String>> {
        let url: &str = "mysql://root:@localhost:3306/mws_database";
    
        let pool: Pool = Pool::new(url).unwrap();

        let conn = pool.get_conn().unwrap();

        let mut outer_v: Vec<Vec<String>> = Vec::new();

        let cut_rows: Vec<Row> = s.fetch(conn).unwrap();

        if cut_rows.len() == 0 {
            outer_v = vec![vec!["0".to_string()]]
        } else {
            for row in cut_rows {

                let mut v: Vec<String> = Vec::new();
                
                for i in 0..n {
                    v.push(row.get_opt::<String, usize>(i).unwrap().unwrap().to_string());
                }
                outer_v.push(v.clone());
            }
        }
        outer_v
    }
}