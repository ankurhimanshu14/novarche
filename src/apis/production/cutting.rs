pub mod cutting {

    use chrono::NaiveDate;
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug, Clone)]
    pub struct Cutting {
        pub planned_date: NaiveDate,
        pub machine: String,
        pub part_code: String,
        pub steel_code: String,
        pub heat_no: String,
        pub planned_qty: usize,
        pub actual_qty: Option<usize>,
        pub ok_qty: usize,
        pub end_pc_wt: Option<f64>
    }

    impl Cutting {
        pub fn new(
            planned_date: NaiveDate,
            machine: String,
            part_code: String,
            steel_code: String,
            heat_no: String,
            planned_qty: usize,
            actual_qty: Option<usize>,
            ok_qty: usize,
            end_pc_wt: Option<f64>
        ) -> Self {
            Cutting {
                planned_date,
                machine,
                part_code,
                steel_code,
                heat_no,
                planned_qty,
                actual_qty,
                ok_qty,
                end_pc_wt
            }
        }

        pub fn post(&self) -> Result<()> {

            let temp_table = "CREATE TEMPORARY TABLE cutting_temp(
                temp_id             INT             NOT NULL            PRIMARY KEY             AUTO_INCREMENT,
                planned_date        DATETIME        NOT NULL,
                machine             VARCHAR(10)     NOT NULL,
                part_code           VARCHAR(20)     NOT NULL,
                steel_code          VARCHAR(20)     NOT NULL,
                heat_no             VARCHAR(20)     NOT NULL,
                planned_qty         INT             NOT NULL,
                actual_qty          INT,
                ok_qty              INT,
                rej_qty             INT             DEFAULT             (actual_qty - ok_qty),
                end_pc_wt           FLOAT(10,3)
            )ENGINE = InnoDB;";

            let url: &str = "mysql://root:@localhost:3306/mws_database";

            let pool: Pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(temp_table)?;

            let insert = "INSERT INTO cutting_temp(
                planned_date,
                machine,
                part_code,
                steel_code,
                heat_no,
                planned_qty,
                actual_qty,
                ok_qty,
                end_pc_wt
            ) VALUES (
                :planned_date,
                :machine,
                :part_code,
                :steel_code,
                :heat_no,
                :planned_qty,
                :actual_qty,
                ok_qty,
                end_pc_wt
            );";

            conn.exec_drop(
                insert,
                params! {
                    "planned_date" => self.planned_date.clone(),
                    "machine" => self.machine.clone(),
                    "part_code" => self.part_code.clone(),
                    "steel_code" => self.steel_code.clone(),
                    "heat_no" => self.heat_no.clone(),
                    "planned_qty" => self.planned_qty.clone(),
                    "actual_qty" => match self.actual_qty.clone() {
                        Some(v) => v,
                        None => 0
                    },
                    "ok_qty" => self.ok_qty.clone(),
                    "end_pc_wt" => self.end_pc_wt.clone()
                }
            )?;

            let cutting_table = "CREATE TABLE IF NOT EXISTS cutting
            (   
                cutting_id             INT             NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                planned_date           DATETIME        NOT NULL,
                machine                VARCHAR(10)     NOT NULL,
                part_no                INT             NOT NULL,
                heat_no                VARCHAR(20)     NOT NULL,
                grade                  VARCHAR(20)     NOT NULL,
                size                   INT             NOT NULL,
                section                VARCHAR(10)     NOT NULL,
                cut_wt                 FLOAT(6,3)      NOT NULL,
                planned_qty            INT             NOT NULL,
                actual_qty             INT,
                ok_qty                 INT,
                rej_qty                INT              DEFAULT          (actual_qty - ok_qty),
                ok_wt                  FLOAT(10,3)      DEFAULT          (ok_qty * cut_wt),
                rej_wt                 FLOAT(10,3)      DEFAULT          (rej_qty * cut_wt),
                end_pc_wt              FLOAT(10,3),
                total_wt               FLOAT(10,3)      DEFAULT          (actual_qty * cut_wt),
                created_at             DATETIME        NOT NULL        DEFAULT             CURRENT_TIMESTAMP,
                modified_at            DATETIME                        ON UPDATE           CURRENT_TIMESTAMP
            )ENGINE = InnoDB;";

            let insert = "INSERT INTO cutting(planned_date, machine, part_no, heat_no, grade, size, section, cut_wt, planned_qty, actual_qty, ok_qty, end_pc_wt)
            SELECT
            c.planned_date,
            c.machine,
            p.part_no,
            g.heat_no,
            s.grade,
            s.size,
            s.section,
            p.cut_wt,
            c.planned_qty,
            c.actual_qty,
            c.ok_qty,
            c.end_pc_wt
            FROM cutting_temp c
            INNER JOIN part p
            ON p.part_code = c.part_code
            INNER JOIN gate_entry g
            ON g.heat_no = c.heat_no
            INNER JOIN steels s
            ON s.steel_code = c.steel_code;";

            conn.query_drop(cutting_table)?;
            conn.query_drop(insert)?;

            Ok(())
        }
    }
}