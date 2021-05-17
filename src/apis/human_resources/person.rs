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
                middle_name: match &middle_name.clone().unwrap().len() {
                    0 => None,
                    _ => Some(middle_name.clone().unwrap())
                },
                last_name,
                gender,
                date_of_birth,
                pri_contact_no: match &pri_contact_no.clone().unwrap().len() {
                    0 => None,
                    _ => Some(pri_contact_no.clone().unwrap())
                },
                sec_contact_no: match &sec_contact_no.clone().unwrap().len() {
                    0 => None,
                    _ => Some(sec_contact_no.clone().unwrap())
                },
                personal_email: match &personal_email.clone().unwrap().len() {
                    0 => None,
                    _ => Some(personal_email.clone().unwrap())
                },
                per_address,
                com_address: match &com_address.clone().unwrap().len() {
                    0 => None,
                    _ => Some(com_address.clone().unwrap())
                },
                pan,
                uidai,
                uan
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
                }
            )?;

            Ok(())
        }

        // pub fn get_person_list() -> Result<Vec<Person>> {
        //     let query = "SELECT * FROM person;";

        //     let url = "mysql://root:@localhost:3306/mws_database".to_string();

        //     let pool = Pool::new(url)?;

        //     let conn = pool.get_conn()?;

        //     let result: Vec<Row> = query.fetch(conn)?;

        //     let mut v1: Vec<Person> = Vec::new();

        //     for entries in result.iter() {
        //         let length: &usize = &entries.len();
        //         let mut v2: Vec<String> = Vec::new();
        //         for index in 0..*length {
        //             let val = &entries.get_opt::<String, usize>(index).unwrap();

        //             match val {
        //                 Ok(_) => v2.push(val.as_ref().unwrap().to_string()),
        //                 Err(_) => v2.push("".to_string()),
        //             }
        //         }
        //         v1.push(v2);
        //     }
        //     Ok(v1)
        // }
    }
}
