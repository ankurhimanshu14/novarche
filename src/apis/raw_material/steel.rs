pub mod steel {

    use chrono::{ NaiveDate };
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug)]
    pub struct Steel {
        pub grade: String,
        pub item_code: String,
        pub section_size: usize,
        pub section_type: String,
    }

    impl Steel {
        pub fn new(
            grade: String,
            item_code: String,
            section_size: usize,
            section_type: String
        ) -> Self {
            Steel {
                grade,
                item_code,
                section_size,
                section_type
            }
        }

        pub fn post(s: Steel) -> Result<()> {
            let table = "CREATE TABLE IF NOT EXISTS steel (
                id                      INT             NOT NULL            PRIMARY KEY         AUTO_INCREMENT,
                grade                   VARCHAR(10)     NOT NULL            UNIQUE,
                item_code               VARCHAR(20)     NOT NULL            UNIQUE,
                section_size            INT             NOT NULL,
                section_type            VARCHAR(3)      NOT NULL,
                created_at              DATETIME        NOT NULL            DEFAULT             CURRENT_TIMESTAMP,
                modified_at             DATETIME                            ON UPDATE           CURRENT_TIMESTAMP
            ) ENGINE=InnoDB;";

            let insert = "INSERT INTO steel (
                grade,
                item_code,
                section_size,
                section_type
            ) VALUES (
                :grade,
                :item_code,
                :section_size,
                :section_type
            );";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            conn.exec_drop(insert, params! {
                "grade" => s.grade,
                "item_code" => s.item_code,
                "section_size" => s.section_size,
                "section_type" => s.section_type
            })?;

            Ok(())
        }
    }
}