pub mod gate_entry {

    use chrono::NaiveDate;
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug, Clone)]
    pub struct GateEntry {
        pub challan_no: usize,
        pub challan_date: NaiveDate,
        pub item_code: String,
        pub item_description: String,
        pub party_code: String,
        pub heat_no: String,
        pub received_qty: f64,
        pub uom: String,
        pub unit_cost: Option<f64>,
        pub total_cost: f64
    }

    impl GateEntry {
        pub fn new(
            challan_no: usize,
            challan_date: NaiveDate,
            item_code: String,
            item_description: String,
            party_code: String,
            heat_no: String,
            received_qty: f64,
            uom: String,
            unit_cost: Option<f64>
        ) -> Self {
            GateEntry {
                challan_no,
                challan_date,
                item_code,
                item_description,
                party_code,
                heat_no,
                received_qty,
                uom,
                unit_cost,
                total_cost: match unit_cost {
                    None => 0.0,
                    Some(_) => unit_cost.unwrap() * received_qty
                }
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
                item_description TEXT,
                party_code      VARCHAR(10)     NOT NULL,
                heat_no         VARCHAR(20)     NOT NULL,
                received_qty    FLOAT(20, 3)    NOT NULL,
                uom             VARCHAR(5)      NOT NULL,
                unit_cost       FLOAT(20, 3),
                total_cost      FLOAT(20, 3)    DEFAULT         (received_qty * unit_cost),
                created_at          DATETIME    NOT NULL            DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                        ON UPDATE           CURRENT_TIMESTAMP,
                UNIQUE INDEX    ch_heatno_itmcd                 (challan_no, heat_no, item_code),
                CONSTRAINT      sr_fk_grn_prty  FOREIGN KEY(party_code)     REFERENCES        party(party_code)         ON UPDATE CASCADE ON DELETE CASCADE,
                CONSTRAINT      sr_fk_grn_itm   FOREIGN KEY(item_code)      REFERENCES        steels(item_code)         ON UPDATE CASCADE ON DELETE CASCADE
            )ENGINE = InnoDB;";

            conn.query_drop(table)?;

            let insert = "INSERT INTO gate_entry(
                challan_no,
                challan_date,
                item_code,
                item_description,
                party_code,
                heat_no,
                received_qty,
                uom,
                unit_cost
            ) VALUES (
                :challan_no,
                :challan_date,
                :item_code,
                :item_description,
                :party_code,
                :heat_no,
                :received_qty,
                :uom,
                :unit_cost
            )";

            conn.exec_drop(
                insert,
                params! {
                    "challan_no" => self.challan_no.clone(),
                    "challan_date" => self.challan_date.clone(),
                    "item_code" => self.item_code.clone(),
                    "item_description" => self.item_description.clone(),
                    "party_code" => self.party_code.clone(),
                    "heat_no" => self.heat_no.clone(),
                    "received_qty" => self.received_qty.clone(),
                    "uom" => self.uom.clone(),
                    "unit_cost" => self.unit_cost.clone()
                }
            )?;

            Ok(())
        }

        pub fn get_heat_no_list() -> Result<Vec<String>> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;
    
            let mut conn = pool.get_conn()?;
            
            let query = "SELECT heat_no FROM gate_entry;";
    
            let mut v: Vec<String> = Vec::new();

            let if_exist = "SELECT COUNT(*)
                FROM information_schema.tables 
                WHERE table_schema = DATABASE()
                AND table_name = 'gate_entry';";

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
                        |heat_no: String| {
                            v.push(heat_no.to_string())
                        }
                    ).unwrap()
                }
            };
            
            Ok(v)
        }

        pub fn assign_approvals(h: String, v: Vec<usize>) -> Result<()> {

            let url: &str = "mysql://root:@localhost:3306/mws_database";

            let pool: Pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            let table = "CREATE TABLE IF NOT EXISTS approved_components(
                approval_id         INT         NOT NULL                    PRIMARY KEY             AUTO_INCREMENT,
                heat_no             VARCHAR(20) NOT NULL,
                part_no             INT         NOT NULL,
                created_at          DATETIME    NOT NULL                    DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                                ON UPDATE           CURRENT_TIMESTAMP,
                UNIQUE INDEX        heat_part                                                 (heat_no, part_no)
            )ENGINE = InnoDB;";

            let insert = "INSERT INTO approved_components(
                heat_no,
                part_no
            ) VALUES (
                :heat_no,
                :part_no
            );";

            conn.query_drop(table)?;

            for p in v {
                conn.exec_drop(
                    insert,
                    params! {
                        "heat_no" => h.to_string(),
                        "part_no" => p
                    }
                )?;
            }

            Ok(())
        }
    }
}