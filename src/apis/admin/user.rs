pub mod user {
    use serde::{ Serialize, Deserialize};
    use std::str;
    use chrono::{ DateTime, Utc };
    use bcrypt::{ hash, DEFAULT_COST, verify };
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

    fn verify_user(username: String, password: String) -> Result<bool> {
        let user = User::get_one(username)?;
        let v = verify(password, &user[0].password).unwrap();
        Ok(v)
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
            conn.exec_drop(insert, (u.email, u.username, u.password, u.role))?;
            Ok(())
        }

        pub fn get() -> Result<Vec<User>> {
            let url = "mysql://root:@localhost:3306/mws_database";

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            let all_users = conn
                                .query_map(
                                    "SELECT email, username, role, password FROM user;",
                                    |(email, username, role, password)| {
                                        User { email, username, role, password }
                                    },
                                )?;
            Ok(all_users)
        }

        pub fn get_one(username: String) -> Result<Vec<User>> {

            let query = format!("SELECT email, username, role, password FROM user WHERE username = '{}';", username);
            let url = "mysql://root:@localhost:3306/mws_database";

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            let user = conn
                            .query_map(
                                query,
                                |(email, username, role, password)| {
                                    User{ email, username, role, password }
                                },
                            )?;

            Ok(user)
        }

        pub fn change_password(u: String, p: String) -> Result<()> {
            let verified_user = verify_user(u.clone(), p.clone()).unwrap();

            if verified_user {
                let mut new_password = String::new();
                println!("Enter new password:");
                std::io::stdin()
                                .read_line(&mut new_password)
                                .expect("Failed to read input");
                let hashed = hash(new_password.to_string().trim_end_matches("\r\n").to_string(), DEFAULT_COST).unwrap();

                let query = format!("UPDATE user SET password = '{}' WHERE username = '{}';", hashed.to_string(), u.to_string());

                let url = "mysql://root:@localhost:3306/mws_database";

                let pool = Pool::new(url)?;
    
                let mut conn = pool.get_conn()?;

                conn.query_drop(query)?;
                
            } else {
                println!("You are not authorized to change the password.");
            }

            Ok(())
        }

        pub fn change_role(u: String, p: String) -> Result<()> {
            let verified_user = verify_user(u.clone(), p.clone()).unwrap();

            if verified_user {
                let mut new_role = String::new();
                println!("Enter new role:");
                std::io::stdin()
                                .read_line(&mut new_role)
                                .expect("Failed to read input");

                let query = format!("UPDATE user SET role = '{}' WHERE username = '{}';", new_role.to_string(), u.to_string());

                let url = "mysql://root:@localhost:3306/mws_database";

                let pool = Pool::new(url)?;
    
                let mut conn = pool.get_conn()?;

                conn.query_drop(query)?;
                
            } else {
                println!("You are not authorized to change the role.");
            }

            Ok(())
        }
    }
}