pub mod gate_entry {

    use chrono::NaiveDate;
    use mysql::*;
    use mysql::prelude::*;
    use uuid::Uuid;

    use crate::apis::utility_tools::parse::parse::parse_from_row;

    #[derive(Debug, Clone)]
    pub struct GateEntry {
        pub gate_entry_id: String,
        pub challan_no: usize,
        pub challan_date: NaiveDate,
        pub steel_code: String,
        pub item_description: String,
        pub party_code: String,
        pub heat_no: String,
        pub heat_code: Option<String>,
        pub received_qty: f64
    }

    impl GateEntry {
        /// Creates a new Gate Entry.
        /// This assigns a Universally unique Identifier utilizing the Uuid crate version v4.
        /// The generated uuid is converted to String value before initializing the struct.
        /// The heat code field identified by heat_code is an optional value.
        pub fn new(
            challan_no: usize,
            challan_date: NaiveDate,
            steel_code: String,
            item_description: String,
            party_code: String,
            heat_no: String,
            heat_code: Option<String>,
            received_qty: f64
        ) -> Self {

            GateEntry {
                gate_entry_id: Uuid::new_v4().to_string(),
                challan_no,
                challan_date,
                steel_code,
                item_description,
                party_code,
                heat_no,
                heat_code: match &heat_code.clone().unwrap().len() {
                    0 => None,
                    _ => Some(heat_code.clone().unwrap())
                },
                received_qty
            }
        }

        /// Saves the new Gate Entry to the MySQL database
        /// The gate entry table references the party and steel databases.
        pub fn post(&self) -> Result<()> {
            let url: &str = "mysql://root:@localhost:3306/mws_database";

            let pool: Pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            let table = "CREATE TABLE IF NOT EXISTS gate_entry(
                grn             INT             NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                gate_entry_id   VARCHAR(200)    NOT NULL        UNIQUE,
                challan_no      BIGINT          NOT NULL,
                challan_date    DATETIME        NOT NULL,
                steel_code       VARCHAR(20)     NOT NULL,
                item_description TEXT,
                party_code      VARCHAR(10)     NOT NULL,
                heat_no         VARCHAR(20)     NOT NULL,
                heat_code       VARCHAR(10),
                received_qty    FLOAT(20, 3)    NOT NULL,
                avail_qty       FLOAT(20, 3)    NOT NULL,
                created_at      DATETIME        NOT NULL            DEFAULT             CURRENT_TIMESTAMP,
                modified_at     DATETIME                            ON UPDATE           CURRENT_TIMESTAMP,
                UNIQUE INDEX    ch_heatno_itmcd                 (challan_no, heat_no, steel_code),
                CONSTRAINT      sr_fk_grn_prty  FOREIGN KEY(party_code)     REFERENCES        party(party_code)         ON UPDATE CASCADE ON DELETE CASCADE,
                CONSTRAINT      sr_fk_grn_itm   FOREIGN KEY(steel_code)     REFERENCES        steels(steel_code)        ON UPDATE CASCADE ON DELETE CASCADE
            )ENGINE = InnoDB;";

            conn.query_drop(table)?;

            let insert = "INSERT INTO gate_entry(
                gate_entry_id,
                challan_no,
                challan_date,
                steel_code,
                item_description,
                party_code,
                heat_no,
                heat_code,
                received_qty,
                avail_qty
            ) VALUES (
                :gate_entry_id,
                :challan_no,
                :challan_date,
                :steel_code,
                :item_description,
                :party_code,
                :heat_no,
                :heat_code,
                :received_qty,
                :avail_qty
            )";

            conn.exec_drop(
                insert,
                params! {
                    "gate_entry_id" => self.gate_entry_id.clone(),
                    "challan_no" => self.challan_no.clone(),
                    "challan_date" => self.challan_date.clone(),
                    "steel_code" => self.steel_code.clone(),
                    "item_description" => self.item_description.clone(),
                    "party_code" => self.party_code.clone(),
                    "heat_no" => self.heat_no.clone(),
                    "heat_code" => self.heat_code.clone(),
                    "received_qty" => self.received_qty.clone(),
                    "avail_qty" => self.received_qty.clone()
                }
            )?;

            Ok(())
        }

        /// Generates the Gate Entry List ordered by challan date.
        pub fn get_gate_entry_list() -> Vec<GateEntry> {
            let query = "SELECT gate_entry_id, challan_no, challan_date, steel_code, item_description, party_code, heat_no, heat_code, received_qty FROM gate_entry ORDER BY challan_date ASC;";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url).unwrap();

            let mut conn = pool.get_conn().unwrap();

            let mut v: Vec<GateEntry> = Vec::new();

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
                        |(gate_entry_id, challan_no, challan_date, steel_code, item_description, party_code, heat_no, heat_code, received_qty)| {

                            let gr = GateEntry {
                                gate_entry_id, challan_no, challan_date, steel_code, item_description, party_code, heat_no, heat_code, received_qty
                            };

                            v.push(gr);
                        }
                    ).unwrap()
                }
            };
            
            v
        }

        /// Returns the Total Received Quantity in Kgs.
        pub fn get_total_received_qty() -> Result<f64> {

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;
    
            let mut conn = pool.get_conn()?;

            let query = "SELECT SUM(received_qty) FROM gate_entry;";

            let mut v: Vec<f64> = Vec::new();

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
                        |total_receiving: f64| {
                            v.push(total_receiving)
                        }
                    ).unwrap()
                }
            };

            Ok(v[0])
        }

        pub fn get_heat_no_list() -> Result<Vec<String>> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;
    
            let mut conn = pool.get_conn()?;
            
            let query = "SELECT DISTINCT heat_no FROM gate_entry;";
    
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

            let temp_table = "CREATE TABLE IF NOT EXISTS temp_approvals(
                approval_id         INT         NOT NULL                    PRIMARY KEY             AUTO_INCREMENT,
                heat_no             VARCHAR(20) NOT NULL,
                part_no             INT         NOT NULL
            )ENGINE = InnoDB;";

            conn.query_drop(temp_table)?;

            let insert = "INSERT INTO temp_approvals(
                heat_no,
                part_no
            ) VALUES (
                :heat_no,
                :part_no
            );";

            let table = "CREATE TABLE IF NOT EXISTS approved_components(
                approval_id         INT         NOT NULL                    PRIMARY KEY             AUTO_INCREMENT,
                rm_id               VARCHAR(100) NOT NULL,
                heat_no             VARCHAR(20) NOT NULL,
                part_no             INT         NOT NULL,
                avail_qty           INT         NOT NULL,
                created_at          DATETIME    NOT NULL                    DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                                ON UPDATE           CURRENT_TIMESTAMP,
                UNIQUE INDEX        heat_part                                                 (heat_no, part_no),
                CONSTRAINT          sr_fk_ap_grn    FOREIGN KEY(rm_id)            REFERENCES      gate_entry(gate_entry_id)       ON UPDATE CASCADE ON DELETE CASCADE
            )ENGINE = InnoDB;";

            let trig = "CREATE TRIGGER after_approved_components_insert
            AFTER INSERT
            ON temp_approvals FOR EACH ROW
            INSERT INTO approved_components (rm_id, heat_no, part_no, avail_qty)
            SELECT
            g.gate_entry_id,
            NEW.heat_no,
            NEW.part_no,
            g.received_qty
            FROM gate_entry g
            WHERE g.heat_no = NEW.heat_no;";

            conn.query_drop(table)?;

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

            match trig_name.contains(&"after_approved_components_insert".to_string()) {
                true => {
                    for p in v {
                        conn.exec_drop(
                            insert,
                            params! {
                                "heat_no" => h.to_string(),
                                "part_no" => p
                            }
                        )?;
                    };
                },
                false => {
                    conn.query_drop(trig)?;
                    for p in v {
                        conn.exec_drop(
                            insert,
                            params! {
                                "heat_no" => h.to_string(),
                                "part_no" => p
                            }
                        )?;
                    };
                }
            }

            Ok(())
        }

        pub fn get_approved_parts(h: String) -> Vec<usize> {
            let query = format!("SELECT part_no FROM approved_components WHERE heat_no = '{}';", h);
    
            let url = "mysql://root:@localhost:3306/mws_database".to_string();
    
            let pool = Pool::new(url).unwrap();
    
            let mut conn = pool.get_conn().unwrap();
    
            let mut v: Vec<usize> = Vec::new();
    
            let if_exist = "SELECT COUNT(*)
                FROM information_schema.tables 
                WHERE table_schema = DATABASE()
                AND table_name = 'approved_components';";
    
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
                        |part_no: usize| {
    
                            v.push(part_no);
                        }
                    ).unwrap()
                }
            };
            
            v
        }

        pub fn get_approved_heats(p: usize) -> Vec<String> {
            let query = format!("SELECT DISTINCT heat_no FROM approved_components WHERE part_no = '{}';", p);
    
            let url = "mysql://root:@localhost:3306/mws_database".to_string();
    
            let pool = Pool::new(url).unwrap();
    
            let mut conn = pool.get_conn().unwrap();
    
            let mut v: Vec<String> = Vec::new();
    
            let if_exist = "SELECT COUNT(*)
                FROM information_schema.tables 
                WHERE table_schema = DATABASE()
                AND table_name = 'approved_components';";
    
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
    
                            v.push(heat_no.to_string());
                        }
                    ).unwrap()
                }
            };
            
            v
        }

        pub fn get_avail_qty(h: String) -> f64 {
            let query = format!("SELECT SUM(avail_qty) FROM gate_entry WHERE heat_no = '{}';", h.to_string());

            let url = "mysql://root:@localhost:3306/mws_database".to_string();
    
            let pool = Pool::new(url).unwrap();
    
            let mut conn = pool.get_conn().unwrap();

            let avail_qty = conn.query_map(
                query,
                |v: Row| {
                    v
                }
            ).unwrap();

            parse_from_row(&avail_qty[0])[0].parse::<f64>().unwrap()
        }

        pub fn get_next_avail_supply(h: String, pl_wt: f64) -> Vec<usize> {

            let query1 = format!("SET @total_avail := (SELECT SUM(avail_qty) FROM gate_entry WHERE heat_no = '{0}');", &h.to_string());
            let query2 = format!("SELECT challan_no FROM gate_entry WHERE heat_no = '{0}' AND @total_avail >= '{1}' ORDER BY challan_date;", h.to_string(), pl_wt);

            let url = "mysql://root:@localhost:3306/mws_database".to_string();
    
            let pool = Pool::new(url).unwrap();
    
            let mut conn = pool.get_conn().unwrap();

            conn.query_drop(query1).unwrap();
            let avail_heat = conn.query_map(
                query2,
                |v: Row| {
                    v
                }
            ).unwrap();

            match &avail_heat.len() {
                0 => vec![0],
                _ => {
                    let mut ch_vec: Vec<usize> = Vec::new();

                    for entries in &avail_heat {
                        let parsed = parse_from_row(entries)[0].to_string().parse::<usize>().unwrap();

                        ch_vec.push(parsed);
                    }
                    ch_vec
                }
            }
        }

        pub fn check_availability(h: String, w: f64) -> bool {
            let query = format!("SELECT SUM(avail_qty) FROM gate_entry WHERE heat_no = '{}'", h.to_string());

            let url = "mysql://root:@localhost:3306/mws_database".to_string();
    
            let pool = Pool::new(url).unwrap();
    
            let mut conn = pool.get_conn().unwrap();

            let avail_qty = conn.query_map(
                query,
                |v: Row| {
                    v
                }
            ).unwrap();

            parse_from_row(&avail_qty[0])[0].parse::<f64>().unwrap() >= w
        }

        pub fn update_by_ch_no(tot_wt: f64, c: usize) -> Result<()> {
            let query = format!("UPDATE gate_entry SET avail_qty = (avail_qty - '{}') WHERE challan_no = '{}' LIMIT 1;", tot_wt, c);

            let url = "mysql://root:@localhost:3306/mws_database".to_string();
    
            let pool = Pool::new(url).unwrap();
    
            let mut conn = pool.get_conn().unwrap();

            conn.query_drop(query)?;

            Ok(())
        }
    }
}