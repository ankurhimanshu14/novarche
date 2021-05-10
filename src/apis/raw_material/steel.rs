pub mod steel {

    use chrono::{ NaiveDate };
    use mysql::*;
    use mysql::prelude::*;

    use crate::apis::raw_material::grades::grades::Grades;
    use crate::apis::raw_material::section::section::Section;

    #[derive(Debug)]
    pub struct Steel {
        pub item_code: String,
        pub grade: Grades,
        pub section: Section
    }

    impl Steel {
        pub fn new(
            item_code: String,
            grade: Grades,
            section: Section
        ) -> Self {
            Steel {
                item_code,
                grade,
                section
            }
        }

        pub fn assign(g: Grades, s: Section) -> Result<()> {
            let steel_table = format!("CREATE TABLE steel
            (   
                steel_id            INT             NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                item_code           VARCHAR(15)     NOT NULL        UNIQUE,
                created_at          DATETIME        NOT NULL        DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                        ON UPDATE           CURRENT_TIMESTAMP
            )
            INSERT INTO steel (item_code, grade, sec_size, sec_type)
                SELECT
                g.grade,
                s.sec_size,
                s.sec_type
                FROM grades g
                INNER JOIN section s ON g.grade = '{}', s.sec_size = '{}', s.sec_type = '{}'
                ORDER BY item_code;", g.grade, s.sec_size, s.sec_type
            );

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(steel_table)?;

            Ok(())
        }
    }
}