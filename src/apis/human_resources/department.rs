pub mod department {

    use chrono::{ NaiveDate };
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug)]
    pub struct Department {
        department_code: String,
        description: String,
        email: String
    }

    impl Department {
        pub fn new(department_code: String, description: String, email: String) -> Self {
            Department {
                department_code,
                description,
                email,
            }
        }

        pub fn post(d: Department) -> Result<()> {
            let table = "CREATE TABLE IF NOT EXISTS department (
                id                      INT             NOT NULL            PRIMARY KEY         AUTO_INCREMENT,
                department_code         VARCHAR(4)      NOT NULL            UNIQUE,
                description             VARCHAR(200)    NOT NULL,
                email                   VARCHAR(50)     NOT NULL            UNIQUE,
                created_at              DATETIME        NOT NULL            DEFAULT             CURRENT_TIMESTAMP,
                modified_at             DATETIME                            ON UPDATE           CURRENT_TIMESTAMP
            ) ENGINE=InnoDB;";

            let insert = "INSERT INTO department (
                department_code,
                description,
                email
            ) VALUES (
                :department_code,
                :description,
                :email
            );";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            conn.exec_drop(insert, params! {
                "department_code" => d.department_code,
                "description" => d.description,
                "email" => d.email
            })?;

            Ok(())
        }

        pub fn find_by_dept_code(q: String) -> Result<Vec<Vec<String>>> {
            let query = format!("SELECT * FROM department WHERE department_code = '{}';", q);

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let conn = pool.get_conn()?;

            let result: Vec<Row> = query.fetch(conn)?;

            let mut v1: Vec<Vec<String>> = Vec::new();

            for entries in result.iter() {
                let length: &usize = &entries.len();
                let mut v2: Vec<String> = Vec::new();
                for index in 0..*length {
                    let val = &entries.get_opt::<String, usize>(index).unwrap();

                    match val {
                        Ok(_) => v2.push(val.as_ref().unwrap().to_string()),
                        Err(_) => v2.push("".to_string()),
                    }
                }
                v1.push(v2);
            }
            Ok(v1)
        }
    }
}