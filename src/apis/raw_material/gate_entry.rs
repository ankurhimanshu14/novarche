pub mod gate_entry {

    use mysql::*;
    use mysql::prelude::*;
    use chrono::NaiveDate;

    #[derive(Debug, Clone)]
    pub struct GateEntry {
        pub grn: usize,
        pub challan_no: usize,
        pub challan_date: NaiveDate,
        pub item_code: String,
        pub heat_no: String,
        pub heat_code: Option<String>,
        pub received_qty: usize
    }

    impl GateEntry {
        pub fn new(
            grn: usize,
            challan_no: usize,
            challan_date: NaiveDate,
            item_code: String,
            heat_no: String,
            heat_code: Option<String>,
            received_qty: usize
        ) -> Self {

            GateEntry {
               grn,
               challan_no,
               challan_date,
               item_code,
               heat_no,
               heat_code: match &heat_code.clone().unwrap().to_string().len() {
                   0 => None,
                   _ => heat_code
               },
               received_qty,
            }
        }
            

        pub fn post(&self) -> Result<()> {
            let table = r"CREATE TABLE IF NOT EXISTS grn_entry(
                id                  INT             NOT NULL               PRIMARY KEY         AUTO_INCREMENT,
                grn                 BIGINT          NOT NULL               UNIQUE,
                challan_no          BIGINT          NOT NULL,
                challan_date        DATETIME        NOT NULL,
                item_code           VARCHAR(50)     NOT NULL,
                heat_no             VARCHAR(10)     NOT NULL,
                heat_code           VARCHAR(10),
                received_qty        INT             NOT NULL,
                created_at          DATETIME        NOT NULL                DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                                ON UPDATE           CURRENT_TIMESTAMP,
                CONSTRAINT sr_fk_grn_itemcode       FOREIGN KEY(item_code)  REFERENCES          steel(item_code) ON DELETE CASCADE ON UPDATE CASCADE,
            ) ENGINE = InnoDB;";

            let insert = "INSERT INTO department (
                grn,
                challan_no,
                challan_date,
                item_code,
                heat_no,
                heat_code,
                received_qty
            ) VALUES (
                :grn,
                :challan_no,
                :challan_date,
                :item_code,
                :heat_no,
                :heat_code,
                :received_qty
            );";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            conn.exec_drop(insert, params! {
                "grn" => self.grn.clone(),
                "challan_no" => self.challan_no.clone(),
                "challan_date" => self.challan_date.clone(),
                "item_code" => self.item_code.clone(),
                "heat_no" => self.heat_no.clone(),
                "heat_code" => self.heat_code.clone(),
                "received_qty" => self.received_qty.clone()
            })?;

            Ok(())
        }
    }
}