pub mod inventory {
    
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug, Clone)]
    pub struct Inventory {
        pub heat_no: String,
        pub grade: String,
        pub size: usize,
        pub section: String,
        pub avail_qty: f64
    }

    impl Inventory {

        pub fn inventory() -> Result<Vec<Inventory>> {

            let url = "mysql://root:@localhost:3306/mws_database".to_string();
    
            let pool = Pool::new(url).unwrap();
    
            let mut conn = pool.get_conn().unwrap();

            let table = "CREATE TEMPORARY TABLE inventory(
            inv_id          INT             NOT NULL            PRIMARY KEY             AUTO_INCREMENT,
            heat_no         VARCHAR(20)     NOT NULL            UNIQUE,
            grade           VARCHAR(20)     NOT NULL,
            size            INT             NOT NULL,
            section             VARCHAR(10)     NOT NULL,
            avail_qty       FLOAT(10, 3)    NOT NULL
            )ENGINE = InnoDB;";

            conn.query_drop(table)?;

            let insert = "INSERT INTO inventory(heat_no, grade, size, section, avail_qty)
            SELECT DISTINCT
            a.heat_no,
            s.grade,
            s.size,
            s.section,
            a.avail_qty
            FROM approved_components a
            INNER JOIN steels s
            ON s.steel_code = (SELECT steel_code FROM gate_entry WHERE gate_entry_id = a.rm_id);";
            
            conn.query_drop(insert)?;

            let select = "SELECT heat_no, grade, size, section, avail_qty FROM inventory WHERE avail_qty <> 0 GROUP BY heat_no;";

            let mut v: Vec<Inventory> = Vec::new();

            conn.query_map(
                select,
                |(heat_no, grade, size, section, avail_qty)| {
                    let inv = Inventory {
                        heat_no, grade, size, section, avail_qty
                    };

                    v.push(inv)
                }
            )?;

            Ok(v)
        }
    }
}