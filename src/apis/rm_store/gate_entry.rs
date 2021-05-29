pub mod gate_entry {

    use chrono::NaiveDate;
    use mysql::*;
    use mysql::prelude::*;

    use crate::apis::utility_tools::parse::parse::parse_from_row;

    #[derive(Debug, Clone)]
    pub struct GateEntry {
        pub challan_no: usize,
        pub challan_date: NaiveDate,
        pub steel_code: String,
        pub item_description: String,
        pub party_code: String,
        pub heat_no: String,
        pub received_qty: f64,
        pub avail_qty: f64
    }

    impl GateEntry {
        pub fn new(
            challan_no: usize,
            challan_date: NaiveDate,
            steel_code: String,
            item_description: String,
            party_code: String,
            heat_no: String,
            received_qty: f64,
        ) -> Self {
            GateEntry {
                challan_no,
                challan_date,
                steel_code,
                item_description,
                party_code,
                heat_no,
                received_qty,
                avail_qty: received_qty.clone()
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
                steel_code       VARCHAR(20)     NOT NULL,
                item_description TEXT,
                party_code      VARCHAR(10)     NOT NULL,
                heat_no         VARCHAR(20)     NOT NULL,
                received_qty    FLOAT(20, 3)    NOT NULL,
                avail_qty       FLOAT(20, 3)    NOT NULL,
                created_at      DATETIME        NOT NULL            DEFAULT             CURRENT_TIMESTAMP,
                modified_at     DATETIME                            ON UPDATE           CURRENT_TIMESTAMP,
                UNIQUE INDEX    ch_heatno_itmcd                 (challan_no, heat_no, steel_code),
                CONSTRAINT      sr_fk_grn_prty  FOREIGN KEY(party_code)     REFERENCES        party(party_code)         ON UPDATE CASCADE ON DELETE CASCADE,
                CONSTRAINT      sr_fk_grn_itm   FOREIGN KEY(steel_code)      REFERENCES        steels(steel_code)         ON UPDATE CASCADE ON DELETE CASCADE
            )ENGINE = InnoDB;";

            conn.query_drop(table)?;

            let insert = "INSERT INTO gate_entry(
                challan_no,
                challan_date,
                steel_code,
                item_description,
                party_code,
                heat_no,
                received_qty,
                avail_qty
            ) VALUES (
                :challan_no,
                :challan_date,
                :steel_code,
                :item_description,
                :party_code,
                :heat_no,
                :received_qty,
                :avail_qty
            )";

            conn.exec_drop(
                insert,
                params! {
                    "challan_no" => self.challan_no.clone(),
                    "challan_date" => self.challan_date.clone(),
                    "steel_code" => self.steel_code.clone(),
                    "item_description" => self.item_description.clone(),
                    "party_code" => self.party_code.clone(),
                    "heat_no" => self.heat_no.clone(),
                    "received_qty" => self.received_qty.clone(),
                    "avail_qty" => self.received_qty.clone()
                }
            )?;

            Ok(())
        }

        pub fn get_gate_entry_list() -> Vec<GateEntry> {
            let query = "SELECT challan_no, challan_date, steel_code, item_description, party_code, heat_no, received_qty, avail_qty FROM gate_entry ORDER BY challan_date DESC;";

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
                        |(challan_no, challan_date, steel_code, item_description, party_code, heat_no, received_qty, avail_qty)| {

                            let gr = GateEntry {
                                challan_no, challan_date, steel_code, item_description, party_code, heat_no, received_qty, avail_qty
                            };

                            v.push(gr);
                        }
                    ).unwrap()
                }
            };
            
            v
        }

        pub fn get_heat_no_list() -> Result<Vec<String>> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;
    
            let mut conn = pool.get_conn()?;
            
            let query = "SELECT heat_no FROM gate_entry GROUP BY heat_no;";
    
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
            let query = format!("SELECT heat_no FROM approved_components WHERE part_no = '{}';", p);
    
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