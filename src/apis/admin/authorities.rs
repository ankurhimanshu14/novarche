pub mod authorities {
    
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug, Clone)]
    pub struct Authorities {
        pub activity: String,
    }

    impl Authorities {
        pub fn new(activity: String) -> Self {
            Authorities {
                activity
            }
        }

        pub fn post(self) -> Result<()> {
            let table = r"CREATE TABLE IF NOT EXISTS authorities(
                activity_id         INT          NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                activity            VARCHAR(50)  NOT NULL        UNIQUE,
                created_at          DATETIME     NOT NULL        DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                     ON UPDATE           CURRENT_TIMESTAMP
            ) ENGINE = InnoDB;";

            let insert = r"INSERT INTO authorities(
                activity
            ) VALUES (
                :activity
            );";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            conn.exec_drop(insert, params! {
                "activity" => self.activity.clone()
            })?;

            Ok(())
        }

        pub fn get() -> Result<Vec<String>> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;
    
            let mut conn = pool.get_conn()?;
            
            let query = "SELECT activity FROM authorities;";
    
            let mut v: Vec<String> = Vec::new();
            
            conn.query_map(
                query,
                |activity: String| {
                    v.push(activity.to_string())
                }
            )?;

            Ok(v)
        }

        pub fn assign(s: String) -> Result<()> {
            let query = format!("CREATE TABLE role_activity
            AS (SELECT
            r.roles_name,
            a.activity
            FROM authorities a
            INNER JOIN roles r
            ON a.activity = '{}');", s);

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(query)?;

            Ok(())
        }
    }
}