pub mod user_roles {
    use mysql::*;
    use mysql::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Roles {
        pub role_name: String,
        pub role_desc: String
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserRoles {
        pub user_account_id: String,
        pub roles: Roles
    }

    impl Roles {
        pub fn new(
            role_name: String,
            role_desc: String
        ) -> Self {
            Roles {
                role_name,
                role_desc
            }
        }

        pub fn post(&self) -> Result<()> {
            let table = r"CREATE TABLE IF NOT EXISTS roles(
                role_id          INT         NOT NULL            PRIMARY KEY         AUTO_INCREMENT,
                role_name        VARCHAR(10) NOT NULL,
                role_desc        TEXT        NOT NULL,
                created_at              DATETIME        NOT NULL            DEFAULT             CURRENT_TIMESTAMP,
                modified_at             DATETIME                            ON UPDATE           CURRENT_TIMESTAMP
            )ENGINE= InnoDB;";

            let insert = r"INSERT INTO roles(
                role_name,
                role_desc
            ) VALUES(
                :role_name,
                :role_desc
            );";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            conn.exec_drop(insert, params! {
                "role_name" => self.role_name.clone(),
                "role_desc" => self.role_desc.clone()
            })?;

            Ok(())
        }
    }

    impl UserRoles {
        pub fn new(
            user_account_id: String,
            roles: Roles
        ) -> Self {
            UserRoles {
                user_account_id,
                roles
            }
        }

        pub fn post(self) -> Result<()> {
            let table = r"CREATE TABLE IF NOT EXISTS user_roles (
                id                      INT             NOT NULL            PRIMARY KEY         AUTO_INCREMENT,
                user_account_id         VARCHAR(20)     NOT NULL,
                roles                   INT             NOT NULL,
                created_at              DATETIME        NOT NULL            DEFAULT             CURRENT_TIMESTAMP,
                modified_at             DATETIME                            ON UPDATE           CURRENT_TIMESTAMP,
                CONSTRAINT sr_fk_rl_id  FOREIGN KEY(user_account_id)        REFERENCES          user(username),
                CONSTRAINT sr_fk_rl_usr FOREIGN KEY(roles)                  REFERENCES          roles(role_id)
            ) ENGINE = InnoDB;";

            let query = "SELECT u.username, r.role_id
            INTO user_roles
            FROM user u
            LEFT JOIN roles r;";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            let result: Vec<Row> = query.fetch(conn)?;

            Ok(())
        }
    }
}