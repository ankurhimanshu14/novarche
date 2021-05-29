pub mod part {

    use chrono::NaiveDate;
    use mysql::prelude::*;
    use mysql::*;

    use crate::apis::utility_tools::parse::parse::parse_from_row;

    #[derive(Debug)]
    pub struct Part {
        pub part_code: String,
        pub part_no: usize,
        pub part_name: String,
        pub grade: String,
        pub sec_size: usize,
        pub sec_type: String,
        pub forging_wt: f32,
        pub cut_wt: f32,
        pub del_cond: String,
        pub drawing_rev_no: String,
        pub drawing_rev_date: NaiveDate
    }

    impl Part {
        pub fn new(
            part_code: String,
            part_no: usize,
            part_name: String,
            grade: String,
            sec_size: usize,
            sec_type: String,
            forging_wt: f32,
            cut_wt: f32,
            del_cond: String,
            drawing_rev_no: String,
            drawing_rev_date: String
        ) -> Self {
            Part {
                part_code,
                part_no,
                part_name,
                grade,
                sec_size,
                sec_type,
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
                part_code           VARCHAR(20)     NOT NULL                UNIQUE,
                part_no             INT             NOT NULL                UNIQUE,
                part_name           TEXT            NOT NULL,
                grade               VARCHAR(10)     NOT NULL,
                sec_size            INT             NOT NULL,
                sec_type            VARCHAR(5)      NOT NULL,
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
                part_code,
                part_no,
                part_name,
                forging_wt,
                grade,
                sec_size,
                sec_type,
                cut_wt,
                del_cond,
                drawing_rev_no,
                drawing_rev_date
            ) VALUES (
                :part_code,
                :part_no,
                :part_name,
                :forging_wt,
                :grade,
                :sec_size,
                :sec_type,
                :cut_wt,
                :del_cond,
                :drawing_rev_no,
                :drawing_rev_date
            );";

            let result = conn.exec_drop(insert, params!{
                "part_code" => self.part_code.clone(),
                "part_no" => self.part_no.clone(),
                "part_name" => self.part_name.clone(),
                "forging_wt" => self.forging_wt.clone(),
                "grade" => self.grade.clone(),
                "sec_size" => self.sec_size.clone(),
                "sec_type" => self.sec_type.clone(),
                "cut_wt" => self.cut_wt.clone(),
                "del_cond" => self.del_cond.clone(),
                "drawing_rev_no" => self.drawing_rev_no.clone(),
                "drawing_rev_date" => self.drawing_rev_date.clone()
            })?;

            assert_eq!(result, ());

            Ok(())
        }

        pub fn get_part_list() -> Vec<Part> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url).unwrap();
    
            let mut conn = pool.get_conn().unwrap();
            
            let query = "SELECT 
            part_code,
            part_no,
            part_name,
            grade,
            sec_size,
            sec_type,
            forging_wt,
            cut_wt,
            del_cond,
            drawing_rev_no,
            drawing_rev_date FROM part;";
    
            let mut v: Vec<Part> = Vec::new();

            let if_exist = "SELECT COUNT(*)
                FROM information_schema.tables 
                WHERE table_schema = DATABASE()
                AND table_name = 'part';";

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
                        |(part_code, part_no, part_name, grade, sec_size, sec_type, forging_wt, cut_wt, del_cond, drawing_rev_no, drawing_rev_date)| {
                            let part = Part {
                                part_code,
                                part_no,
                                part_name,
                                grade,
                                sec_size,
                                sec_type,
                                forging_wt,
                                cut_wt,
                                del_cond,
                                drawing_rev_no,
                                drawing_rev_date
                            };
                            v.push(part)
                        }
                    ).unwrap()
                }
            };
            
            v
        }

        pub fn find_part_code(p: usize) -> Vec<String> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url).unwrap();
    
            let mut conn = pool.get_conn().unwrap();

            let query = format!("SELECT part_code FROM part WHERE part_no = '{}';", p);

            let mut v: Vec<String> = Vec::new();

            let if_exist = "SELECT COUNT(*)
                FROM information_schema.tables 
                WHERE table_schema = DATABASE()
                AND table_name = 'part';";

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
                        |part_code: String| {
                            v.push(part_code.to_string())
                        }
                    ).unwrap()
                }
            };

            v
        }

        pub fn get_steel(p: usize) -> Vec<(String, usize, String)> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url).unwrap();
    
            let mut conn = pool.get_conn().unwrap();

            let query = format!("SELECT grade, sec_size, sec_type FROM part WHERE part_no = '{}';", p);

            let mut v: Vec<(String, usize, String)> = Vec::new();

            let if_exist = "SELECT COUNT(*)
                FROM information_schema.tables 
                WHERE table_schema = DATABASE()
                AND table_name = 'part';";

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
                        |(grade, sec_size, sec_type): (String, usize, String)| {
                            v.push((grade, sec_size, sec_type))
                        }
                    ).unwrap()
                }
            };

            v
        }

        pub fn get_cut_wt(p: usize) -> f64 {
            let query = format!("SELECT cut_wt FROM part WHERE part_no = '{}';", p);

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url).unwrap();
    
            let mut conn = pool.get_conn().unwrap();

            let cut_wt = conn.query_map(
                query,
                |v: Row| {
                    v
                }
            ).unwrap();

            parse_from_row(&cut_wt[0])[0].parse::<f64>().unwrap()
        }

        pub fn fetch_cut_wt(pc: String) -> f64 {
            let query = format!("SELECT cut_wt FROM part WHERE part_code = '{}';", pc);

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url).unwrap();
    
            let mut conn = pool.get_conn().unwrap();

            let cut_wt = conn.query_map(
                query,
                |v: Row| {
                    v
                }
            ).unwrap();

            parse_from_row(&cut_wt[0])[0].parse::<f64>().unwrap()
        }
    }
}