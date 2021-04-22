pub mod login {

    use chrono::{ NaiveDate };
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug, Clone)]
    pub struct Login {
        logger_id: String,
        login_token: String,
        pub start_time: NaiveDateTime,
        pub end_time: NaiveDateTime
    }

    impl Login {
        pub fn new() -> Self {
            Login {
                logger_id: 
            }
        }
    }
}