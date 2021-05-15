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

            let if_exist = "SELECT COUNT(*)
                FROM information_schema.tables 
                WHERE table_schema = DATABASE()
                AND table_name = 'authorities';";

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
                        |activity: String| {
                            v.push(activity.to_string())
                        }
                    ).unwrap()
                }
            };
            
            Ok(v)
        }

        pub fn assign(r: String, s: String) -> Result<()> {
            let table = "CREATE TABLE IF NOT EXISTS role_activity(
                role_act_id         INT             NOT NULL        PRIMARY KEY     AUTO_INCREMENT,
                roles_name          VARCHAR(20)     NOT NULL,
                activity            VARCHAR(50)     NOT NULL,
                created_at          DATETIME     NOT NULL        DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                     ON UPDATE           CURRENT_TIMESTAMP,
                UNIQUE INDEX unique_rolename_activity (roles_name, activity)
            )ENGINE = InnoDB;";

            let query = format!("INSERT INTO role_activity(roles_name, activity)
            SELECT
            r.roles_name,
            a.activity
            FROM authorities a
            INNER JOIN roles r
            ON a.activity = '{}' AND r.roles_name = '{}'
            ORDER BY r.roles_name;", s.to_string(), r.to_string());

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            conn.query_drop(query)?;

            Ok(())
        }

        pub fn delete(activity: String) -> Result<()> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;
    
            let mut conn = pool.get_conn()?;

            let query1 = format!("DELETE FROM authorities WHERE activity = '{}';", activity);

            let query2 = format!("DELETE 
                r.roles_name, a.activity
                FROM 
                    roles r
                    INNER JOIN activity a
                    ON a.activity = '{}'
                    ORDER BY r.roles_name;",
                    activity
            );

            conn.query_drop(query1)?;

            conn.query_drop(query2)?;

            Ok(())
        }
    }
}