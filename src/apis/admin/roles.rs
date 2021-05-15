pub mod roles {
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug, Clone)]
    pub struct Roles {
        pub roles_name: String
    }

    impl Roles {
        pub fn new(roles_name: String) -> Self {
            Roles {
                roles_name
            }
        }

        pub fn post(self) -> Result<()> {
            let table = r"CREATE TABLE IF NOT EXISTS roles(
                roles_id            INT             NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                roles_name          VARCHAR(20)     NOT NULL        UNIQUE,
                created_at          DATETIME        NOT NULL        DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                        ON UPDATE           CURRENT_TIMESTAMP
            ) ENGINE = InnoDB;";

            let insert = r"INSERT INTO roles(
                roles_name
            ) VALUES (
                :roles_name
            );";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            conn.exec_drop(insert, params! {
                "roles_name" => self.roles_name.clone()
            })?;

            Ok(())
        }

        pub fn get() -> Result<Vec<String>> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;
    
            let mut conn = pool.get_conn()?;
            
            let query = "SELECT roles_name FROM roles;";
    
            let mut v: Vec<String> = Vec::new();

            let if_exist = "SELECT COUNT(*)
                FROM information_schema.tables 
                WHERE table_schema = DATABASE()
                AND table_name = 'roles';";

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
                        |roles_name: String| {
                            v.push(roles_name.to_string())
                        }
                    ).unwrap()
                }
            };
            
            Ok(v)
        }

        pub fn delete(role: String) -> Result<()> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;
    
            let mut conn = pool.get_conn()?;

            let query = format!("DELETE FROM roles WHERE roles_name = '{}';", role);

            conn.query_drop(query)?;

            Ok(())
        }
    }
}