pub mod user {
    use serde::{ Serialize, Deserialize};
    use chrono::{ DateTime, Utc };
    use bcrypt::{ DEFAULT_COST, hash, verify };
    use std::convert::From;
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum Role {
        Root,
        General
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct User {
        pub email: String,
        pub username: String,
        password: String,
        pub role: Role,
        created_at: DateTime<Utc>,
        modified_at: Option<DateTime<Utc>>
    }   
}