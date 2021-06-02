pub mod user_signup {

    use mysql::*;
    use mysql::prelude::*;
    use bcrypt::{ hash, DEFAULT_COST };

    #[derive(Debug, Clone)]
    pub struct User {
        pub employee_id: String,
        pub username: String,
        pub hash: String,
        pub role: String
    }

    impl User {
        pub fn new(
            employee_id: String,
            username: String,
            password: String,
            role: String
        ) -> Self {
            User {
                employee_id,
                username,
                hash: hash(password, DEFAULT_COST).unwrap(),
                role
            }
        }

        pub fn sign_up(self) -> Result<u64> {
            let table = "CREATE TEMPORARY TABLE IF NOT EXISTS user_details(
                user_id             INT             NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                username            VARCHAR(20)     NOT NULL        UNIQUE,
                hash                VARCHAR(200)    NOT NULL,
                created_at          DATETIME        NOT NULL        DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                        ON UPDATE           CURRENT_TIMESTAMP
            ) Engine = InnoDB;";

            let insert = r"INSERT INTO user_details(
                username,
                hash
            ) VALUES (
                :username,
                :hash
            );";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            conn.exec_drop(insert, params! {
                "username" => self.username.clone(),
                "hash" => self.hash.clone()
            })?;

            let user_table = "CREATE TABLE IF NOT EXISTS user
            (   
                user_id             INT             NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                employee_id         VARCHAR(10)     NOT NULL        UNIQUE,
                username            VARCHAR(20)     NOT NULL        UNIQUE,
                hash                VARCHAR(200)    NOT NULL,
                roles_name          VARCHAR(20)     NOT NULL,
                status              VARCHAR(20)     NOT NULL        DEFAULT             'ACTIVE',
                created_at          DATETIME        NOT NULL        DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                        ON UPDATE           CURRENT_TIMESTAMP,
                UNIQUE INDEX        username_emp_id     (username, employee_id)
            )ENGINE = InnoDB;";

            let insert = format!("INSERT INTO user (employee_id, username, hash, roles_name)
                SELECT
                e.employee_id,
                u.username,
                u.hash,
                r.roles_name
                FROM employee e
                INNER JOIN user_details u
                ON e.employee_id = '{0}' AND u.username = '{1}' AND u.hash = '{2}'
                INNER JOIN roles r
                ON r.roles_name = '{3}'
                ORDER BY user_id;", self.employee_id, self.username, self.hash, self.role
            );

            conn.query_drop(user_table)?;

            conn.query_drop(insert)?;

            Ok(conn.last_insert_id())
        }

        pub fn default() -> Result<u64> {
            User::new("0000".to_string(), "root".to_string(), "root".to_string(), "ROOT".to_string()).sign_up()
        }

        pub fn change_password(new_password: String, username: String) -> Result<()> {

            let hash = hash(new_password, DEFAULT_COST).unwrap();

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            let query = format!("UPDATE user
            SET user.hash = '{0}'
            WHERE user.username = '{1}';", hash, username);

            conn.query_drop(query)?;

            Ok(())
        }
    }
}