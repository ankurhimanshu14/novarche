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
        pub fn new() -> Self {
            let mut first_name = String::new();
            println!("Enter firstname:");
            std::io::stdin()
                .read_line(&mut first_name)
                .expect("Failed to read input");

            let mut middle_name = String::new();
            println!("Enter middlename:");
            std::io::stdin()
                .read_line(&mut middle_name)
                .expect("Failed to read input");

            let mut last_name = String::new();
            println!("Enter lastname:");
            std::io::stdin()
                .read_line(&mut last_name)
                .expect("Failed to read input");

            let mut gender = String::new();
            println!("Enter Gender:");
            std::io::stdin()
                .read_line(&mut gender)
                .expect("Failed to read input");

            let mut dob = String::new();
            println!("Enter date of birth (dd-mm--yyyy):");
            std::io::stdin()
                .read_line(&mut dob)
                .expect("Failed to read input");

            let mut pri_contact_no = String::new();
            println!("Enter Primary Contact No:");
            std::io::stdin()
                .read_line(&mut pri_contact_no)
                .expect("Failed to read input");

            let mut sec_contact_no = String::new();
            println!("Enter Secondary Contact No:");
            std::io::stdin()
                .read_line(&mut sec_contact_no)
                .expect("Failed to read input");

            let mut personal_email = String::new();
            println!("Enter Personal Email ID:");
            std::io::stdin()
                .read_line(&mut personal_email)
                .expect("Failed to read input");

            let mut per_address = String::new();
            println!("Enter Permanent Address:");
            std::io::stdin()
                .read_line(&mut per_address)
                .expect("Failed to read input");

            let mut com_address = String::new();
            println!("Enter Communication Address:");
            std::io::stdin()
                .read_line(&mut com_address)
                .expect("Failed to read input");

            let mut pan = String::new();
            println!("Enter PAN:");
            std::io::stdin()
                .read_line(&mut pan)
                .expect("Failed to read input");

            let mut uidai = String::new();
            println!("Enter UIDAI:");
            std::io::stdin()
                .read_line(&mut uidai)
                .expect("Failed to read input");

            let mut uan = String::new();
            println!("Enter UAN:");
            std::io::stdin()
                .read_line(&mut uan)
                .expect("Failed to read input");

            Person {
                first_name: first_name.to_string().trim_end_matches("\r\n").to_string(),

                middle_name: match middle_name
                    .to_string()
                    .trim_end_matches("\r\n")
                    .to_string()
                    .len()
                {
                    0 => None,
                    _ => Some(middle_name.to_string().trim_end_matches("\r\n").to_string()),
                },

                last_name: last_name.to_string().trim_end_matches("\r\n").to_string(),

                gender: gender.to_string().trim_end_matches("\r\n").to_string(),

                date_of_birth: NaiveDate::parse_from_str(
                    &dob.to_string().trim_end_matches("\r\n").to_string(),
                    "%d-%m-%Y",
                )
                .unwrap(),

                pri_contact_no: match pri_contact_no
                    .to_string()
                    .trim_end_matches("\r\n")
                    .to_string()
                    .len()
                {
                    0 => None,
                    _ => Some(
                        pri_contact_no
                            .to_string()
                            .trim_end_matches("\r\n")
                            .to_string(),
                    ),
                },

                sec_contact_no: match sec_contact_no
                    .to_string()
                    .trim_end_matches("\r\n")
                    .to_string()
                    .len()
                {
                    0 => None,
                    _ => Some(
                        sec_contact_no
                            .to_string()
                            .trim_end_matches("\r\n")
                            .to_string(),
                    ),
                },

                personal_email: match personal_email
                    .to_string()
                    .trim_end_matches("\r\n")
                    .to_string()
                    .len()
                {
                    0 => None,
                    _ => Some(
                        personal_email
                            .to_string()
                            .trim_end_matches("\r\n")
                            .to_string(),
                    ),
                },

                per_address: per_address.to_string().trim_end_matches("\r\n").to_string(),

                com_address: match com_address
                    .to_string()
                    .trim_end_matches("\r\n")
                    .to_string()
                    .len()
                {
                    0 => None,
                    _ => Some(com_address.to_string().trim_end_matches("\r\n").to_string()),
                },

                pan: pan.to_string().trim_end_matches("\r\n").to_string(),

                uidai: uidai
                    .to_string()
                    .trim_end_matches("\r\n")
                    .to_string()
                    .parse()
                    .unwrap(),

                uan: match uan.to_string().trim_end_matches("\r\n").to_string().len() {
                    0 => None,
                    _ => Some(
                        uan.to_string()
                            .trim_end_matches("\r\n")
                            .to_string()
                            .parse()
                            .unwrap(),
                    ),
                },
            }
        }

        pub fn post(p: &Person) -> Result<()> {
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
                    "first_name" => p.first_name.clone(),
                    "middle_name" => p.middle_name.clone(),
                    "last_name" => p.last_name.clone(),
                    "gender" => p.gender.clone(),
                    "date_of_birth" => p.date_of_birth.clone(),
                    "pri_contact_no" => p.pri_contact_no.clone(),
                    "sec_contact_no" => p.sec_contact_no.clone(),
                    "personal_email" => p.personal_email.clone(),
                    "per_address" => p.per_address.clone(),
                    "com_address" => p.com_address.clone(),
                    "pan" => p.pan.clone(),
                    "uidai" => p.uidai.clone(),
                    "uan" => p.uan.clone(),
                },
            )?;

            Ok(())
        }
    }
}
