pub mod department {

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

        pub fn post(self) -> Result<()> {
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
                "department_code" =>self.department_code,
                "description" =>self.description,
                "email" =>self.email
            })?;

            Ok(())
        }

        pub fn get_list() -> Result<Vec<Department>> {
            let query = "SELECT department_code, description, email FROM department;";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;
            
            let mut v: Vec<Department> = Vec::new();

            let if_exist = "SELECT COUNT(*)
                FROM information_schema.tables 
                WHERE table_schema = DATABASE()
                AND table_name = 'department';";

            let result = conn.query_map(
                if_exist,
                |count: usize| {
                    count
                }
            ).unwrap();

            match &result[0] {
                0 => vec![()],
                _ => {
                    conn.query_map(
                        query,
                        |(department_code, description, email)| {
                            let dep = Department{
                                department_code,
                                description,
                                email
                            };

                            v.push(dep)
                        }
                    ).unwrap()
                }
            };
            
            Ok(v)
        }

        pub fn get_dept_code_list() -> Result<Vec<String>> {
            let mut v: Vec<String> = Vec::new();

            let dep_vec: Vec<Department> = Department::get_list().unwrap();

            for d in dep_vec {
                v.push(d.department_code)
            }

            Ok(v)
        }
    }
}