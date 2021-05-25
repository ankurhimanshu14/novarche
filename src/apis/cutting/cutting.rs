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
        pub actual_qty: usize
    };

    impl Cutting {
        pub fn new(
            planned_date: NaiveDate,
            machine: String,
            part_code: usize,
            steel_code: String,
            heat_no: String,
            planned_qty: usize,
            actual_qty: usize
        ) -> Self {
            Cutting {
                planned_date,
                machine,
                part_code,
                steel_code,
                heat_no,
                planned_qty,
                actual_qty
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
                actual_qty          INT             NOT NULL
            )ENGINE = InnoDB;";

            let url: &str = "mysql://root:@localhost:3306/mws_database";

            let pool: Pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(temp_table)?;

            let insert = "INSERT INTO cutting_temp(
                planned_date
                machine,
                part_code,
                steel_code,
                heat_no,
                planned_qty,
                actual_qty
            ) VALUES (
                :planned_date,
                :machine,
                :part_code,
                :steel_code,
                :heat_no,
                :planned_qty,
                :actual_qty
            );";

            conn.exec_drop(
                insert,
                params! {
                    "planned_date" => self.planned_date.clone()
                    "machine" => self.machine.clone(),
                    "part_code" => self.part_code.clone(),
                    "steel_code" => self.steel_code.clone(),
                    "heat_no" => self.heat_no.clone(),
                    "planned_qty" => self.planned_qty.clone(),
                    "actual_qty" => self.actual_qty.clone()
                }
            )?;

            let cuttting_table = "CREATE TABLE IF NOT EXISTS cutting
            (   
                cutting_id             INT             NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                planned_date           DATETIME        NOT NULL,
                machine                VARCHAR(10)     NOT NULL,
                part_no                INT             NOT NULL,
                heat_no                VARCHAR(20)     NOT NULL,
                grade_name             VARCHAR(20)     NOT NULL,
                sec_size               INT             NOT NULL,
                sec_type               VARCHAR(10)     NOT NULL,
                cut_wt                 FLAOT(6,3)      NOT NULL,
                planned_qty            INT             NOT NULL,
                actual_qty             INT,
                total_wt               FLOAT(6,3)      DEFAULT          (cut_wt*actual_qty),
                created_at             DATETIME        NOT NULL        DEFAULT             CURRENT_TIMESTAMP,
                modified_at            DATETIME                        ON UPDATE           CURRENT_TIMESTAMP
            )ENGINE = InnoDB;"

            let insert = "INSERT INTO cutting(planned_date, machine, part_no, heat_no, grade_name, sec_size, sec_type, cut_wt, planned_qty, actual_qty)
            SELECT
            c.planned_date,
            c.machine,
            p.part_no,
            g.heat_no,
            g.grade_name,
            s.sec_size,
            s.sec_type,
            p.cut_wt,
            c.planned_qty,
            c.actual_qty
            FROM cutting_temp c
            INNER JOIN part p
            ON p.part_no = c.
            ";
            Ok(())
        }
    }
}