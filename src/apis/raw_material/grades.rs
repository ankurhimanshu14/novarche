pub mod grades {

    
    use chrono::NaiveDate;
    use mysql::prelude::*;
    use mysql::*;
    
    #[derive(Debug, Clone)]

    pub struct Grades {
        pub grade_name: String
    }

    impl Grades {
        pub fn new(grade_name: String) -> Self {
            Grades {
                grade_name
            }
        }

        pub fn post(&self) -> Result<()> {
            let url: &str = "mysql://root:@localhost:3306/mws_database";

            let pool: Pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            let table = "CREATE TABLE IF NOT EXISTS grades(
                grade_id        INT         NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                grade_name      VARCHAR(20) NOT NULL        UNIQUE,
                created_at      DATETIME    NOT NULL        DEFAULT             CURRENT_TIMESTAMP,
                modified_at     DATETIME                    ON UPDATE           CURRENT_TIMESTAMP
            )ENGINE = InnoDB";

            conn.query_drop(table)?;

            let insert = "INSERT INTO grades(grade_name) VALUES(:grade_name);";

            let result = conn.exec_drop(insert, params!{
                "grade_name" => self.grade_name
            })?;

            Ok(())
        }

        pub fn get_list() -> Result<Vec<String>> {
            let query = "SELECT grade_name FROM grades;";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            let mut v: Vec<Grades> = Vec::new();

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
                        |(grade_name)| {
                            let grd = Grades::new(grade_name);
                            v.push(grd);
                        }
                    ).unwrap()
                }
            };
            
            Ok(v)
        }
    }
}