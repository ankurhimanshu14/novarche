pub mod user {

    use mysql::*;
    use mysql::prelude::*;
    use crate::apis::admin::roles::roles::Roles;
    use crate::apis::admin::authorities::authorities::Authorities;
    use bcrypt::{ hash, verify, DEFAULT_COST };

    #[derive(Debug, Clone)]
    pub struct User {
        pub employee_id: String,
        pub email: String,
        pub username: String,
        pub hash: String,
        pub role: String,
        pub authority: String
    }

    impl User {
        pub fn new(
            employee_id: String,
            email: String,
            username: String,
            password: String,
            role: String,
            authority: String
        ) -> Self {
            User {
                employee_id,
                email,
                username,
                hash: hash(password, DEFAULT_COST).unwrap(),
                role: format!("{}", role),
                authority:format!("{}", authority)
            }
        }

        pub fn post(self) -> Result<()> {
            let table = "CREATE TABLE IF NOT EXISTS user_details(
                user_id             INT             NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                username            VARCHAR(20)     NOT NULL,
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

            let user_table = "CREATE TABLE user
            (   
                user_id             INT             NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                employee_id         VARCHAR(10)     NOT NULL        UNIQUE,
                username            VARCHAR(20)     NOT NULL        UNIQUE,
                hash                VARCHAR(200)    NOT NULL,
                email               VARCHAR(50)     NOT NULL,
                roles_name          VARCHAR(20)     NOT NULL,
                activity            VARCHAR(50)     NOT NULL,
                created_at          DATETIME        NOT NULL        DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                        ON UPDATE           CURRENT_TIMESTAMP,
                UNIQUE INDEX        username_emp_id     (username, employee_id)
            )ENGINE = InnoDB;";

            let insert = format!("INSERT INTO user (employee_id, username, hash, email, roles_name, activity)
                AS SELECT
                e.employee_id,
                u.username,
                u.hash,
                d.email,
                r.roles_name,
                a.activity
                FROM employee e
                INNER JOIN user_details u ON u.username = '{}', u.hash = '{}'
                INNER JOIN department d ON d.email = '{}'
                INNER JOIN authorities a ON a.activity = '{}'
                INNER JOIN roles r ON r.roles_name = '{}'
                ORDER BY user_id;", self.username, self.hash, self.email, self.authority, self.role
            );

            conn.query_drop(user_table)?;

            conn.query_drop(insert)?;

            Ok(())
        }
    }
}