pub mod person {

    use chrono::NaiveDate;
    use mysql::prelude::*;
    use mysql::*;

    #[derive(Debug, Clone)]
    pub struct Person {
        pub first_name: String,
        pub middle_name: Option<String>,
        pub last_name: String,
        pub gender: String,
        date_of_birth: NaiveDate,
        pub pri_contact_no: Option<String>,
        pub sec_contact_no: Option<String>,
        pub personal_email: Option<String>,
        pub per_address: String,
        pub com_address: Option<String>,
        pan: String,
        pub uidai: usize,
        uan: Option<usize>,
    }

    impl Person {
        pub fn new(
            first_name: String,
            middle_name: Option<String>,
            last_name: String,
            gender: String,
            date_of_birth: NaiveDate,
            pri_contact_no: Option<String>,
            sec_contact_no: Option<String>,
            personal_email: Option<String>,
            per_address: String,
            com_address: Option<String>,
            pan: String,
            uidai: usize,
            uan: Option<usize>,
        ) -> Self {
            Person {
                first_name,
                middle_name: match &middle_name.unwrap().len() {
                    0 => None,
                    _ => Some(middle_name.unwrap())
                },
                last_name,
                gender,
                date_of_birth: NaiveDate::parse_from_str(
                    &date_of_birth.to_string(),
                    "%d-%m-%Y",
                ).unwrap(),
                pri_contact_no,
                sec_contact_no,
                personal_email,
                per_address,
                com_address,
                pan,
                uidai,
                uan,
            }
        }

        pub fn post(self) -> Result<()> {
            let table = "CREATE TABLE IF NOT EXISTS person(
                id                  INT             NOT NULL            PRIMARY KEY         AUTO_INCREMENT,
                first_name          VARCHAR(100)    NOT NULL,
                middle_name         VARCHAR(100),
                last_name           VARCHAR(100)    NOT NULL,
                gender              VARCHAR(100)    NOT NULL,
                date_of_birth       DATETIME        NOT NULL,
                pri_contact_no      VARCHAR(12),
                sec_contact_no      VARCHAR(12),
                personal_email      VARCHAR(100),
                per_address         TEXT            NOT NULL,
                com_address         TEXT,
                pan                 VARCHAR(10)     UNIQUE,
                uidai               BIGINT          UNIQUE,
                uan                 BIGINT          UNIQUE,
                created_at          DATETIME        NOT NULL        DEFAULT     CURRENT_TIMESTAMP,
                modified_at         DATETIME                        ON UPDATE   CURRENT_TIMESTAMP         
            ) ENGINE = InnoDB;";

            let insert = r"INSERT INTO person(
                first_name,
                middle_name,
                last_name,
                gender,
                date_of_birth,
                pri_contact_no,
                sec_contact_no,
                personal_email,
                per_address,
                com_address,
                pan,
                uidai,
                uan
            ) VALUES (
                :first_name,
                :middle_name,
                :last_name,
                :gender,
                :date_of_birth,
                :pri_contact_no,
                :sec_contact_no,
                :personal_email,
                :per_address,
                :com_address,
                :pan,
                :uidai,
                :uan
            );";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            conn.exec_drop(
                insert,
                params! {
                    "first_name" => self.first_name.clone(),
                    "middle_name" => self.middle_name.clone(),
                    "last_name" => self.last_name.clone(),
                    "gender" => self.gender.clone(),
                    "date_of_birth" => self.date_of_birth.clone(),
                    "pri_contact_no" => self.pri_contact_no.clone(),
                    "sec_contact_no" => self.sec_contact_no.clone(),
                    "personal_email" => self.personal_email.clone(),
                    "per_address" => self.per_address.clone(),
                    "com_address" => self.com_address.clone(),
                    "pan" => self.pan.clone(),
                    "uidai" => self.uidai.clone(),
                    "uan" => self.uan.clone(),
                },
            )?;

            Ok(())
        }
    }
}
