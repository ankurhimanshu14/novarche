pub mod cutting {

    use chrono::NaiveDate;
    use mysql::*;
    use mysql::prelude::*;
    use uuid::Uuid;

    use crate::apis::engineering::part::part::Part;
    use crate::apis::raw_material::steel::steel::Steel;
    use crate::apis::rm_store::gate_entry::gate_entry::GateEntry;
    use crate::apis::utility_tools::parse::parse::parse_from_row;

    #[derive(Debug, Clone)]
    pub struct Cutting {
        pub cutting_id: String,
        pub planned_date: NaiveDate,
        pub machine: String,
        pub part_code: String,
        pub steel_code: String,
        pub heat_no: String,
        pub planned_qty: usize
    }

    impl Cutting {
        pub fn new(
            planned_date: NaiveDate,
            machine: String,
            part_code: String,
            steel_code: String,
            heat_no: String,
            planned_qty: usize
        ) -> Self {
            Cutting {
                cutting_id: Uuid::new_v4().to_string(),
                planned_date,
                machine,
                part_code,
                steel_code,
                heat_no,
                planned_qty
            }
        }

        pub fn post(&self, pl_wt: f64) -> Result<u64> {

            let temp_table = "CREATE TABLE IF NOT EXISTS cutting_temp(
                temp_id             INT             NOT NULL            PRIMARY KEY             AUTO_INCREMENT,
                cutting_id          VARCHAR(200)    NOT NULL            UNIQUE,
                planned_date        DATETIME        NOT NULL,
                machine             VARCHAR(10)     NOT NULL,
                part_code           VARCHAR(20)     NOT NULL,
                steel_code          VARCHAR(20)     NOT NULL,
                heat_no             VARCHAR(20)     NOT NULL,
                planned_qty         INT             NOT NULL
            )ENGINE = InnoDB;";

            let url: &str = "mysql://root:@localhost:3306/mws_database";

            let pool: Pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(temp_table)?;

            let insert = "INSERT INTO cutting_temp(
                cutting_id,
                planned_date,
                machine,
                part_code,
                steel_code,
                heat_no,
                planned_qty
            ) VALUES (
                :cutting_id,
                :planned_date,
                :machine,
                :part_code,
                :steel_code,
                :heat_no,
                :planned_qty
            );";

            conn.exec_drop(
                insert,
                params! {
                    "cutting_id" => self.cutting_id.clone(),
                    "planned_date" => self.planned_date.clone(),
                    "machine" => self.machine.clone(),
                    "part_code" => self.part_code.clone(),
                    "steel_code" => self.steel_code.clone(),
                    "heat_no" => self.heat_no.clone(),
                    "planned_qty" => self.planned_qty.clone()
                }
            )?;

            let cutting_table = "CREATE TABLE IF NOT EXISTS cutting
            (   
                id                     INT             NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                rm_id                  VARCHAR(100)    NOT NULL,
                prod_id                VARCHAR(100)    NOT NULL,
                planned_date           DATETIME        NOT NULL,
                machine                VARCHAR(10)     NOT NULL,
                part_no                INT             NOT NULL,
                heat_no                VARCHAR(20)     NOT NULL,
                grade                  VARCHAR(20)     NOT NULL,
                size                   INT             NOT NULL,
                section                VARCHAR(10)     NOT NULL,
                cut_wt                 FLOAT(6,3)      NOT NULL,
                planned_qty            INT             NOT NULL,
                actual_qty             INT                             DEFAULT          0,
                ok_qty                 INT                             DEFAULT          0,
                rej_qty                INT                             DEFAULT          (actual_qty - ok_qty),
                ok_wt                  FLOAT(10,3)                     DEFAULT          (ok_qty * cut_wt),
                rej_wt                 FLOAT(10,3)                     DEFAULT          (rej_qty * cut_wt),
                end_pc_wt              FLOAT(10,3),
                total_wt               FLOAT(10,3)                     DEFAULT          (actual_qty * cut_wt),
                created_at             DATETIME         NOT NULL       DEFAULT            CURRENT_TIMESTAMP,
                modified_at            DATETIME                        ON UPDATE          CURRENT_TIMESTAMP,
                CONSTRAINT          sr_fk_cut_rm    FOREIGN KEY(rm_id)            REFERENCES      approved_components(rm_id)       ON UPDATE CASCADE ON DELETE CASCADE
            )ENGINE = InnoDB;";

            let insert = "INSERT INTO cutting(rm_id, prod_id, planned_date, machine, part_no, heat_no, grade, size, section, cut_wt, planned_qty)
            SELECT
            a.rm_id,
            c.cutting_id,
            c.planned_date,
            c.machine,
            p.part_no,
            a.heat_no,
            s.grade,
            s.size,
            s.section,
            p.cut_wt,
            c.planned_qty
            FROM cutting_temp c
            INNER JOIN part p
            ON p.part_code = c.part_code
            INNER JOIN approved_components a
            ON a.heat_no = c.heat_no AND a.part_no = p.part_no
            INNER JOIN steels s
            ON s.steel_code = c.steel_code;";

            conn.query_drop(cutting_table)?;

            conn.query_drop(insert)?;

            Ok(conn.last_insert_id())        
        }

        pub fn update_cutting_status(id: usize, aq: usize, oq: usize, ep: f64) -> Result<()> {
            let stmt = format!("UPDATE cutting
            SET actual_qty = '{0}', ok_qty = '{1}', end_pc_wt = '{2}'
            WHERE cutting_id = '{3}';", aq, oq, ep, id);

            let trig1 = "CREATE TRIGGER before_cutting_update
            BEFORE UPDATE
            ON cutting FOR EACH ROW
                SET new.rej_qty = (new.actual_qty - new.ok_qty), new.ok_wt = (old.cut_wt * new.ok_qty), new.rej_wt = (old.cut_wt * new.rej_qty), new.total_wt = (old.cut_wt * new.actual_qty);";

            let trig2 = "CREATE TRIGGER after_cutting_update
            AFTER UPDATE
            ON cutting FOR EACH ROW
            BEGIN
                SET @total_wt := new.total_wt;
                UPDATE gate_entry SET avail_qty = (avail_qty - @total_wt)
                WHERE old.heat_no = heat_no AND avail_qty >= @total_wt
                LIMIT 1;
            END ;";

            let url: &str = "mysql://root:@localhost:3306/mws_database";
    
            let pool: Pool = Pool::new(url)?;
    
            let mut conn = pool.get_conn()?;

            let result = conn.query_map(
                "SHOW TRIGGERS FROM mws_database;",
                |t: Row| {
                    parse_from_row(&t)
                }
            ).unwrap();

            let mut trig_name: Vec<String> = Vec::new();

            for v in result.clone() {
                trig_name.push(v[0].clone());
            }

            match trig_name.contains(&"before_cutting_update".to_string()) {
                true => {
                    match trig_name.contains(&"after_cutting_update".to_string()) {
                        true => conn.query_drop(stmt)?,
                        false => {
                            conn.query_drop(trig2)?;
                            conn.query_drop(stmt)?;
                        }
                    }
                },
                false => {
                    conn.query_drop(trig1)?;
                    match trig_name.contains(&"after_cutting_update".to_string()) {
                        true => conn.query_drop(stmt)?,
                        false => {
                            conn.query_drop(trig2)?;
                            conn.query_drop(stmt)?;
                        }
                    }
                }
            }

            Ok(())
        }

        pub fn get_cutting_list() -> Vec<Vec<String>> {
            let query = "SELECT cutting_id, planned_date, part_no, heat_no, planned_qty, actual_qty, ok_qty, rej_qty FROM cutting ORDER BY planned_date DESC;";

            let url: &str = "mysql://root:@localhost:3306/mws_database";
    
            let pool: Pool = Pool::new(url).unwrap();
    
            let conn = pool.get_conn().unwrap();

            let mut outer_v: Vec<Vec<String>> = Vec::new();



            let cut_rows: Vec<Row> = query.fetch(conn).unwrap();

            for row in cut_rows {

                let mut v: Vec<String> = Vec::new();
                
                for i in 0..8 {
                    v.push(row.get_opt::<String, usize>(i).unwrap().unwrap().to_string());
                }
                outer_v.push(v.clone());
            }
            outer_v
        }
    }
}