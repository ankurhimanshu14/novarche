pub mod grades {
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug)]
    pub struct Grades {
        pub grade: String
    }

    impl Grades {
        pub fn new(grade: String) -> Self {
            Grades {
                grade
            }
        }

        pub fn post(self) -> Result<()> {
            let table = "CREATE TABLE IF NOT EXISTS grades(
            grade_id        INT         NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
            grade           VARCHAR(10) NOT NULL        UNIQUE,
            created_at      DATETIME    NOT NULL        DEFAULT             CURRENT_TIMESTAMP,
            modified_at     DATETIME                    ON UPDATE           CURRENT_TIMESTAMP
            )ENGINE = InnoDB;";

            let insert = "INSERT INTO grades(grade) VALUES(:grade);";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            conn.exec_drop(insert, params! {
                "grade" => self.grade.clone()
            })?;

            Ok(())
        }

        pub fn get() -> Result<Vec<String>> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;
    
            let mut conn = pool.get_conn()?;
            
            let query = "SELECT grade FROM grades;";
    
            let mut v: Vec<String> = Vec::new();

            let if_exist = "SELECT COUNT(*)
                FROM information_schema.tables 
                WHERE table_schema = DATABASE()
                AND table_name = 'grades';";

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
                        |grade: String| {
                            v.push(grade.to_string())
                        }
                    ).unwrap()
                }
            };
            
            Ok(v)
        }
    }
}