pub mod part {

    use chrono::NaiveDate;
    use mysql::prelude::*;
    use mysql::*;

    #[derive(Debug)]
    pub struct Part {
        pub part_no: usize,
        pub part_name: String,
        pub grade: String,
        pub forging_wt: f32,
        pub cut_wt: f32,
        pub del_cond: String,
        pub drawing_rev_no: String,
        pub drawing_rev_date: NaiveDate
    }

    impl Part {
        pub fn new(
            part_no: usize,
            part_name: String,
            grade: String,
            forging_wt: f32,
            cut_wt: f32,
            del_cond: String,
            drawing_rev_no: String,
            drawing_rev_date: String
        ) -> Self {
            Part {
                part_no,
                part_name,
                grade,
                forging_wt,
                cut_wt,
                del_cond,
                drawing_rev_no,
                drawing_rev_date: NaiveDate::parse_from_str(
                        &drawing_rev_date.to_string().trim_end_matches("\r\n").to_string(),
                        "%d-%m-%Y",
                    )
                    .unwrap(),
            }
        }

        pub fn post(&self) -> mysql::Result<()> {
            let url: &str = "mysql://root:@localhost:3306/mws_database";

            let pool: Pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            let table = "CREATE TABLE IF NOT EXISTS part (
                part_id             INT             NOT NULL                PRIMARY KEY             AUTO_INCREMENT,
                part_no             INT             NOT NULL                UNIQUE,
                part_name           TEXT            NOT NULL,
                grade               VARCHAR(10)     NOT NULL,
                forging_wt          FLOAT(6,3)      NOT NULL,
                cut_wt              FLOAT(6,3),
                del_cond            VARCHAR(20)     NOT NULL,
                drawing_rev_no      VARCHAR(2)      NOT NULL,
                drawing_rev_date    DATETIME        NOT NULL,
                created_at          DATETIME        NOT NULL            DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                            ON UPDATE           CURRENT_TIMESTAMP
            )ENGINE = InnoDB;";

            conn.query_drop(table)?;

            let insert = "INSERT INTO part(
                part_no,
                part_name,
                forging_wt,
                grade,
                cut_wt,
                del_cond,
                drawing_rev_no,
                drawing_rev_date
            ) VALUES (
                :part_no,
                :part_name,
                :forging_wt,
                :grade,
                :cut_wt,
                :del_cond,
                :drawing_rev_no,
                :drawing_rev_date
            );";

            let result = conn.exec_drop(insert, params!{
                "part_no" => self.part_no.clone(),
                "part_name" => self.part_name.clone(),
                "forging_wt" => self.forging_wt.clone(),
                "grade" => self.grade.clone(),
                "cut_wt" => self.cut_wt.clone(),
                "del_cond" => self.del_cond.clone(),
                "drawing_rev_no" => self.drawing_rev_no.clone(),
                "drawing_rev_date" => self.drawing_rev_date.clone()
            })?;

            assert_eq!(result, ());

            Ok(())
        }
    }
}