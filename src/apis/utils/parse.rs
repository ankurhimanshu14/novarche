pub mod parse {

    use mysql::*;

    pub fn parse_from_row(row: &Row) -> Vec<String> {
        let length: &usize = &row.len();

        let mut v: Vec<String> = Vec::new();

        for index in 0..*length {
            let val = row.get_opt::<String, usize>(index).unwrap();

            match val {
                Ok(_) => v.push(val.unwrap()),
                Err(_) => v.push("".to_string())
            }
        }

        v
    }
}