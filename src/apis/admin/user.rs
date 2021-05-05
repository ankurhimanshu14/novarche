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
                role: format!("{:?}", role),
                authority:format!("{:?}", authority)
            }
        }

        pub fn post(self) -> Result<()> {
            let table = format!("CREATE TABLE IF NOT EXISTS user(
                user_id             INT             NOT NULL        PRIMARY KEY         AUTO_INCREMENT,
                username            VARCHAR(20)     NOT NULL        UNIQUE,
                hash                VARCHAR(200)    NOT NULL,
                created_at          DATETIME        NOT NULL        DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                        ON UPDATE           CURRENT_TIMESTAMP
            )
            AS SELECT DISTINCT
            e.employee_id,
            d.email,
            r.roles_name,
            a.activity
            FROM employee e
            INNER JOIN department d, authorities a, roles r
            ON d.email = {} AND a.activity = {} AND r.roles_name = {}
            ORDER BY user_id;", self.email, self.authority, self.role);

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            Ok(())
        }
    }
}