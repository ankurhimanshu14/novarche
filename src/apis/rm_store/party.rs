pub mod party {

    use mysql::*;
    use mysql::prelude::*;

    pub struct Party {
        pub party_code: String,
        pub party_name: String,
        pub party_address: String
    }

    impl Party {
        pub fn new(
            party_code: String,
            party_name: String,
            party_address: String
        ) -> Self {
            Party {
                party_code,
                party_name,
                party_address
            }
        }

        pub fn post(&self) -> Result<()> {
            let table = "CREATE TABLE IF NOT EXISTS party (
                party_id            INT             NOT NULL            PRIMARY KEY                 AUTO_INCREMENT,
                party_code          VARCHAR(10)     NOT NULL            UNIQUE,
                party_name          VARCHAR(100)    NOT NULL,
                party_address       TEXT            NOT NULL,
                created_at          DATETIME        NOT NULL            DEFAULT                     CURRENT_TIMESTAMP,
                modified_at         DATETIME                            ON UPDATE                   CURRENT_TIMESTAMP
            )ENGINE = InnoDB;";

            let insert = "INSERT INTO party (party_code, party_name, party_address) VALUES (:party_code, :party_name, :party_address);";

            let url: &str = "mysql://root:@localhost:3306/mws_database";

            let pool: Pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            conn.exec_drop(
                insert,
                params!{
                    "party_code" => self.party_code.clone(),
                    "party_name" => self.party_name.clone(),
                    "party_address" => self.party_address.clone()
                }
            )?;

            Ok(())
        }

        pub fn get() -> Result<Vec<String>> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;
    
            let mut conn = pool.get_conn()?;
            
            let query = "SELECT party_code, party_name, party_address FROM party;";
    
            let mut v: Vec<String> = Vec::new();

            let if_exist = "SELECT COUNT(*)
                FROM information_schema.tables 
                WHERE table_schema = DATABASE()
                AND table_name = 'party';";

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
                        |(party_code, party_name, party_address)| {

                            let party = Party {
                                party_code,
                                party_name,
                                party_address
                            };
                            
                            v.push(party.party_name.to_string())
                        }
                    ).unwrap()
                }
            };
            
            Ok(v)
        }

        pub fn get_party_code(party_name: &String) -> Vec<String> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url).unwrap();
    
            let mut conn = pool.get_conn().unwrap();
            
            let query = format!("SELECT party_code FROM party WHERE party_name = '{}';", party_name);
    
            let mut v: Vec<String> = Vec::new();

            conn.query_map(
                query,
                |party_code: String| {
                    
                    v.push(party_code.to_string())
                }
            ).unwrap();
            
            v
        }
    }
}