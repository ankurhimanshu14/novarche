pub mod steel {

    use mysql::prelude::*;
    use mysql::*;

    #[derive(Debug, Clone)]
    pub struct Steel {
        pub item_code: String,
        pub grade: String,
        pub size: usize,
        pub section: String
    }

    impl Steel {
        pub fn new(
            item_code: String,
            grade: String,
            size: usize,
            section: String
        ) -> Self {
            Steel {
                item_code,
                grade,
                size,
                section
            }
        }

        pub fn post(&self) -> Result<()> {

            let url: &str = "mysql://root:@localhost:3306/mws_database";

            let pool: Pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            let table = "CREATE TABLE IF NOT EXISTS steels (
                steel_id            INT             NOT NULL            PRIMARY KEY             AUTO_INCREMENT,
                item_code           VARCHAR(20)     NOT NULL            UNIQUE,
                grade               VARCHAR(20)     NOT NULL,
                size                INT             NOT NULL,
                section             VARCHAR(10)     NOT NULL,
                created_at          DATETIME        NOT NULL            DEFAULT                 CURRENT_TIMESTAMP,
                modified_at         DATETIME                            ON UPDATE               CURRENT_TIMESTAMP,
                CONSTRAINT          sr_fk_stl_grd   FOREIGN KEY(grade)  REFERENCES              grades(grade_name)      ON DELETE CASCADE ON UPDATE CASCADE,
                UNIQUE INDEX        grd_size_sec    (grade, size, section)
            )ENGINE = InnoDB";

            let insert = "INSERT INTO steels( item_code, grade, size, section )
            VALUES ( :item_code, :grade, :size, :section );";

            conn.query_drop(table)?;

            conn.exec_drop(insert, params!{
                "item_code" => self.item_code.clone(),
                "grade" => self.grade.clone(),
                "size" => self.size,
                "section" => self.section.clone()
            })?;

            Ok(())
        }

        pub fn get_steel_list() -> Vec<Steel> {
            let query = "SELECT item_code, grade, size, section FROM steels;";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url).unwrap();

            let mut conn = pool.get_conn().unwrap();

            let mut v: Vec<Steel> = Vec::new();

            let if_exist = "SELECT COUNT(*)
                FROM information_schema.tables 
                WHERE table_schema = DATABASE()
                AND table_name = 'steels';";

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
                        |(item_code, grade, size, section)| {

                            let steel = Steel {
                                item_code, grade, size, section
                            };

                            v.push(steel);
                        }
                    ).unwrap()
                }
            };
            
            v
        }
    }
}