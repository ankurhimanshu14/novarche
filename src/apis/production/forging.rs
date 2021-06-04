pub mod forging {

    use chrono::NaiveDate;
    use mysql::*;
    use mysql::prelude::*;

    use crate::apis::utils::{
        row_parser::parser::row_parser,
        gen_uuid::gen_uuid::generate_uuid,
        mysql_commands::mysql_commands::check_table_exists,
    };

    #[derive(Debug, Clone)]
    pub struct Forging {
        pub forging_id: String,
        pub planned_date: NaiveDate,
        pub machine: String,
        pub part_code: String,
        pub planned_qty: usize
    }

    impl Forging {
        pub fn new(
            planned_date: NaiveDate,
            machine: String,
            part_code: String,
            planned_qty: usize
        ) -> Self {
            Forging {
                forging_id: generate_uuid(),
                planned_date,
                machine,
                part_code,
                planned_qty
            }
        }

        pub fn post(&self, c_id: String) -> Result<u64> {
            let temp_table = "CREATE TEMPORARY TABLE forging_temp(
                temp_id             INT             NOT NULL            PRIMARY KEY             AUTO_INCREMENT,
                forging_id          VARCHAR(100)    NOT NULL            UNIQUE,
                planned_date        DATETIME        NOT NULL,
                machine             VARCHAR(20)     NOT NULL,
                part_code           VARCHAR(20)     NOT NULL,
                planned_qty         INT             NOT NULL
            )ENGINE = InnoDB;";

            let url: &str = "mysql://root:@localhost:3306/mws_database";

            let pool: Pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(temp_table)?;

            let insert = "INSERT INTO forging_temp(
                forging_id,
                planned_date,
                machine,
                part_code,
                planned_qty
            ) VALUES (
                :forging_id,
                :planned_date,
                :machine,
                :part_code,
                :planned_qty
            );";

            conn.exec_drop(
                insert,
                params! {
                    "forging_id" => self.forging_id.clone(),
                    "planned_date" => self.planned_date.clone(),
                    "machine" => self.machine.clone(),
                    "part_code" => self.part_code.clone(),
                    "planned_qty" => self.planned_qty.clone()
                }
            )?;

            let forging_table = "CREATE TABLE IF NOT EXISTS forging
            (   
                id                     INT             NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                cutting_id             VARCHAR(100)    NOT NULL,
                forging_id             VARCHAR(100)    NOT NULL,
                planned_date           DATETIME        NOT NULL,
                machine                VARCHAR(20)     NOT NULL,
                part_no                INT             NOT NULL,
                forging_wt             FLOAT(6,3)      NOT NULL,
                planned_qty            INT             NOT NULL,
                actual_qty             INT                             DEFAULT          0,
                ok_qty                 INT                             DEFAULT          0,
                rej_qty                INT                             DEFAULT          (actual_qty - ok_qty),
                issued_qty             INT                             DEFAULT          0,
                ok_wt                  FLOAT(10,3)                     DEFAULT          (ok_qty * forging_wt),
                rej_wt                 FLOAT(10,3)                     DEFAULT          (rej_qty * forging_wt),
                total_wt               FLOAT(10,3)                     DEFAULT          (actual_qty * forging_wt),
                created_at             DATETIME         NOT NULL       DEFAULT            CURRENT_TIMESTAMP,
                modified_at            DATETIME                        ON UPDATE          CURRENT_TIMESTAMP,
                UNIQUE INDEX           cut_forg                                          (cutting_id, forging_id),
                CONSTRAINT          sr_fk_frg_cut    FOREIGN KEY(cutting_id)            REFERENCES      cutting(cutting_id)       ON UPDATE CASCADE ON DELETE CASCADE
            )ENGINE = InnoDB;";

            conn.query_drop(forging_table)?;

            let insert = format!("INSERT INTO forging(cutting_id, forging_id, planned_date, machine, part_no, forging_wt, planned_qty)
            SELECT
            c.cutting_id,
            f.forging_id,
            f.planned_date,
            f.machine,
            p.part_no,
            p.forging_wt,
            f.planned_qty
            FROM forging_temp f
            INNER JOIN part p
            ON p.part_code = f.part_code
            INNER JOIN cutting c
            ON c.part_no = (SELECT part_no FROM part WHERE part_code = f.part_code) AND c.cutting_id = '{}'
            AND c.actual_qty >= f.planned_qty;", c_id);

            conn.query_drop(insert)?;

            Ok(conn.last_insert_id())
        }

        pub fn qty_in_plan(p: usize) -> isize {

            let query = format!("SELECT SUM(planned_qty) FROM forging WHERE part_no = {} AND actual_qty = 0 GROUP BY cutting_id, part_no;", p);

            match check_table_exists("forging".to_string()) {
                Ok(false) => 0,
                Ok(true) => match row_parser(query, "forging".to_string(), 1)[0][0].parse::<isize>().unwrap() {
                    0 => 0,
                    v => v
                },
                Err(_) => -1
            }            
        }

        pub fn booked_qty(c_id: String, p: usize) -> isize {

            let query = format!("SELECT SUM(planned_qty) FROM forging WHERE cutting_id = '{}' AND part_no = {} AND actual_qty = 0 GROUP BY cutting_id, part_no;", c_id, p);

            match check_table_exists("forging".to_string()) {
                Ok(false) => 0,
                Ok(true) => row_parser(query, "forging".to_string(), 1)[0][0].parse::<isize>().unwrap(),
                Err(_) => -1
            }            
        }

        // pub fn update_forging_status(c_id: String, f_id: String, aq: usize, oq: usize) -> Result<()> {
        //     let stmt = format!("UPDATE forging
        //     SET actual_qty = '{0}', ok_qty = '{1}'
        //     WHERE cutting_id = '{2}' AND forging_id = '{3}';", aq, oq, c_id, f_id);

        //     let trig1 = "CREATE TRIGGER before_forging_update
        //     BEFORE UPDATE
        //     ON forging FOR EACH ROW
        //         SET
        //         new.rej_qty = (new.actual_qty - new.ok_qty),
        //         new.ok_wt = (old.forging_wt * new.ok_qty),
        //         new.rej_wt = (old.forging_wt * new.rej_qty),
        //         new.total_wt = (old.forging_wt * new.actual_qty);";

        //     let trig2 = "CREATE TRIGGER after_forging_update
        //     AFTER UPDATE
        //     ON forging FOR EACH ROW
        //     BEGIN
        //         UPDATE cutting SET issued_qty = new.actual_qty
        //         WHERE cutting_id = (SELECT DISTINCT cutting_id FROM forging WHERE forging_id = old.forging_id);
        //     END ;";

        //     let url: &str = "mysql://root:@localhost:3306/mws_database";
    
        //     let pool: Pool = Pool::new(url)?;
    
        //     let mut conn = pool.get_conn()?;

        //     let result = conn.query_map(
        //         "SHOW TRIGGERS FROM mws_database;",
        //         |t: Row| {
        //             parse_from_row(&t)
        //         }
        //     ).unwrap();

        //     let mut trig_name: Vec<String> = Vec::new();

        //     for v in result.clone() {
        //         trig_name.push(v[0].clone());
        //     }

        //     match trig_name.contains(&"before_forging_update".to_string()) {
        //         true => {
        //             match trig_name.contains(&"after_forging_update".to_string()) {
        //                 true => conn.query_drop(stmt)?,
        //                 false => {
        //                     conn.query_drop(trig2)?;
        //                     conn.query_drop(stmt)?;
        //                 }
        //             }
        //         },
        //         false => {
        //             conn.query_drop(trig1)?;
        //             match trig_name.contains(&"after_forging_update".to_string()) {
        //                 true => conn.query_drop(stmt)?,
        //                 false => {
        //                     conn.query_drop(trig2)?;
        //                     conn.query_drop(stmt)?;
        //                 }
        //             }
        //         }
        //     }

        //     Ok(())
        // }
    }
}