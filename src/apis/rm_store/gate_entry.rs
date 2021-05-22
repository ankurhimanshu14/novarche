pub mod gate_entry {

    use chrono::NaiveDate;
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug, Clone)]
    pub struct GateEntry {
        pub grn: usize,
        pub challan_no: usize,
        pub challan_date: NaiveDate,
        pub item_code: String,
        pub party_code: String,
        pub received_qty: f64,
        pub uom: String,
        pub unit_cost: Option<f64>,
        pub total_cost: Option<f64>
    }

    impl GateEntry {
        pub fn new(
            grn: usize,
            challan_no: usize,
            challan_date: NaiveDate,
            item_code: String,
            party_code: String,
            received_qty: f64,
            uom: String,
            unit_cost: Option<f64>,
            total_cost: Option<f64>
        ) -> Self {
            GateEntry {
                grn,
                challan_no,
                challan_date,
                item_code,
                party_code,
                received_qty,
                uom,
                unit_cost,
                total_cost
            }
        }

        pub fn post(&self) -> Result<()> {
            let url: &str = "mysql://root:@localhost:3306/mws_database";

            let pool: Pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            let table = "CREATE TABLE IF NOT EXISTS gate_entry(
                grn             INT             NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                challan_no      BIGINT          NOT NULL,
                challan_date    DATETIME        NOT NULL,
                item_code       VARCHAR(20)     NOT NULL,
                party_code      VARCHAR(10)     NOT NULL,
                received_qty    FLOAT(20, 3)    NOT NULL,
                uom             VARCHAR(5)      NOT NULL,
                unit_cost       FLOAT(20, 3),
                total_cost      FLOAT(20, 3)    DEFAULT         (received_qty * unit_cost),
                CONSTRAINT      sr_fk_grn_itm   FOREIGN KEY(item_code)      REFERENCES        steels(item_code)         ON UPDATE CASCADE ON DELETE CASCADE
            )ENGINE = InnoDB;";

            conn.query_drop(table)?;

            let insert = "INSERT INTO "
        }
    }
}