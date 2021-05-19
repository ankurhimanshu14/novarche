pub mod user_signup {

    use mysql::*;
    use mysql::prelude::*;
    use crate::apis::admin::roles::roles::Roles;
    use crate::apis::admin::authorities::authorities::Authorities;
    use bcrypt::{ hash, verify, DEFAULT_COST };

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

        pub fn sign_up(self) -> Result<()> {
            let table = "CREATE TABLE IF NOT EXISTS user_details(
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
                ON u.username = '{}' AND u.hash = '{}'
                INNER JOIN roles r
                ON r.roles_name = '{}'
                ORDER BY user_id;", self.username, self.hash, self.role
            );

            conn.query_drop(user_table)?;

            conn.query_drop(insert)?;

            Ok(())
        }
    }
}