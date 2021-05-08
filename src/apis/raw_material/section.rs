pub mod section {
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug, Clone)]
    pub enum SectionType {
        RCS,
        DIA
    }

    #[derive(Debug, Clone)]
    pub struct Section {
        pub sec_size: u16,
        pub sec_type: SectionType
    }

    impl Section {
        pub fn new(sec_size: u16, sec_type: SectionType) -> Self {
            Section {
                sec_size,
                sec_type
            }
        }

        pub fn post(self) -> Result<()> {
            let table = "CREATE TABLE IF NOT EXISTS section(
            sec_id        INT         NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
            sec_size      INT         NOT NULL,
            sec_type      VARCHAR(3)  NOT NULL,
            created_at      DATETIME    NOT NULL        DEFAULT             CURRENT_TIMESTAMP,
            modified_at     DATETIME                    ON UPDATE           CURRENT_TIMESTAMP
            )ENGINE = InnoDB;";

            let insert = "INSERT INTO section(sec_size, sec_type) VALUES(:sec_size, :sec_type);";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            conn.exec_drop(insert, params! {
                "sec_size" => self.sec_size.clone(),
                "sec_type" => format!("{:?}", self.sec_type.clone())
            })?;

            Ok(())
        }

        pub fn get() -> Result<Vec<(u16, String)>> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;
    
            let mut conn = pool.get_conn()?;
            
            let query = "SELECT sec_size, sec_type FROM section;";
    
            let mut v: Vec<(u16, String)> = Vec::new();

            let if_exist = "SELECT COUNT(*)
                FROM information_schema.tables 
                WHERE table_schema = DATABASE()
                AND table_name = 'section';";

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
                        |(sec_size, sec_type)| {
                            v.push((sec_size, sec_type))
                        }
                    ).unwrap()
                }
            };
            
            Ok(v)
        }
    }
}