pub mod user {
    use serde::{ Serialize, Deserialize};
    use std::str;
    use chrono::{ DateTime, Utc };
    use bcrypt::{ hash, DEFAULT_COST };
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct User {
        pub email: String,
        pub username: String,
        password: String,
        pub role: String,
        // created_at: DateTime<Utc>,
        // modified_at: Option<DateTime<Utc>>
    }

    impl User {
        pub fn new() -> Self {
            let mut email = String::new();
            println!("Enter Email:");
            std::io::stdin()
                            .read_line(&mut email)
                            .expect("Failed to read input");
            
            let mut username = String::new();
            println!("Enter username:");
            std::io::stdin()
                            .read_line(&mut username)
                            .expect("Failed to read input");

            let mut password = String::new();
            println!("Enter password:");
            std::io::stdin()
                            .read_line(&mut password)
                            .expect("Failed to read input");
            
            let mut role = String::new();
            println!("Enter role:");
            std::io::stdin()
                            .read_line(&mut role)
                            .expect("Failed to read input");
            
            User {
                email: email.to_string()
                                .trim_end_matches("\r\n")
                                .to_string(),
                
                username: username.to_string()
                                .trim_end_matches("\r\n")
                                .to_string(),

                password: hash(password.to_string().trim_end_matches("\r\n").to_string(), DEFAULT_COST).unwrap(),

                role: role.to_string()
                        .trim_end_matches("\r\n")
                        .to_string(),

                // created_at: Utc::now(),

                // modified_at: None
            }
        }

        pub fn post(u: User) -> Result<()> {
            let table = "CREATE TABLE IF NOT EXISTS user(
                id          INT             NOT NULL            PRIMARY KEY AUTO_INCREMENT,
                email       VARCHAR(50)     NOT NULL,
                username    VARCHAR(20)     NOT NULL            UNIQUE,
                password    VARCHAR(100)    NOT NULL,
                role        VARCHAR(20)     NOT NULL
            ) ENGINE = InnoDB;";

            let insert = "INSERT INTO user(email, username, password, role) VALUES (?, ?, ?, ?);";

            let url = "mysql://root:@localhost:3306/mws_database";

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;
            conn.exec_drop(insert, (u.email, u.username, u.password, u.role)).unwrap();
            Ok(())
        }

        pub fn get() -> Result<Vec<User>> {
            let url = "mysql://root:@localhost:3306/mws_database";

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            let all_users = conn
                                .query_map(
                                    "SELECT email, username, role, password FROM user",
                                    |(email, username, role, password)| {
                                        User { email, username, role, password }
                                    },
                                )?;
            Ok(all_users)
        }
    }
}