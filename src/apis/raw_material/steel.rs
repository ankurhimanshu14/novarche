pub mod steel {

    use chrono::{ NaiveDate };
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug)]
    pub struct Steel {
        pub item_code: String,
        pub grade: Grades,
        pub section: Section
    }

    impl Steel {
        pub fn new(
            item_code: String,
            grade: String,
            section: Section
        ) -> Self {
            Steel {
                item_code,
                grade,
                section
            }
        }

        pub fn assign(self) -> Result<()> {
            let query = "CREATE TABLE IF NOT EXISTS steel(
                steel_id            INT         NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                item_code           VARCHAR(15) NOT NULL        UNIQUE,
                created_at          DATETIME    NOT NULL        DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                    ON UPDATE           CURRENT_TIMESTAMP
            )"
                "SELECT g.employee_id, p.first_name, p.middle_name, p.last_name, e.dept_code, p.uan, e. designation, e.reporting_to
                FROM employee e
                LEFT JOIN person p
                ON p.uidai = e.person_id;");

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            conn.exec_drop(insert, params! {
                "grade" => self.grade.clone(),
                "item_code" => self.item_code.clone(),
                "section_size" => self.section_size.clone(),
                "section_type" => self.section_type.clone()
            })?;

            Ok(())
        }
    }
}