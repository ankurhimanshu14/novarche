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
        pub fn new() -> Self {
            let mut department_code = String::new();
            println!("Enter Department Code");
            std::io::stdin()
                            .read_line(&mut department_code)
                            .expect("Failed to read input");

            let mut description = String::new();
            println!("Enter Department Description");
            std::io::stdin()
                            .read_line(&mut description)
                            .expect("Failed to read input");

            let mut email = String::new();
            println!("Enter Department Email ID");
            std::io::stdin()
                            .read_line(&mut email)
                            .expect("Failed to read input");

            Department {
                department_code: department_code.to_string()
                                                .trim_end_matches("\r\n")
                                                .to_string(),

                description: description.to_string()
                                        .trim_end_matches("\r\n")
                                        .to_string(),

                email: email.to_string()
                            .trim_end_matches("\r\n")
                            .to_string(),
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
    }
}