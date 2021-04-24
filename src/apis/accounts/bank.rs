pub mod bank {

    use mysql::prelude::*;
    use mysql::*;

    #[derive(Debug)]
    pub struct Bank {
        bank_name: String,
        bank_ifsc: String,
    }

    impl Bank {
        pub fn new() -> Self {
            let mut bank_name = String::new();
            println!("Enter Bank Name: ");
            std::io::stdin()
                .read_line(&mut bank_name)
                .expect("Failed to read input");

            let mut bank_ifsc = String::new();
            println!("Enter Bank IFSC: ");
            std::io::stdin()
                .read_line(&mut bank_ifsc)
                .expect("Failed to read input");

            Bank {
                bank_name: bank_name.to_string().trim_end_matches("\r\n").to_string(),

                bank_ifsc: bank_ifsc.to_string().trim_end_matches("\r\n").to_string(),
            }
        }

        pub fn post(bank: Bank) -> Result<()> {
            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            let bank_table = "CREATE TABLE IF NOT EXISTS bank (
                id              INT             NOT NULL            PRIMARY KEY         AUTO_INCREMENT,
                bank_name       VARCHAR(100)    NOT NULL,
                bank_ifsc       VARCHAR(20)     NOT NULL            UNIQUE,
                created_at      DATETIME        NOT NULL            DEFAULT             CURRENT_TIMESTAMP,
                modified_at     DATETIME                            ON UPDATE           CURRENT_TIMESTAMP
            ) ENGINE = InnoDB;";

            conn.query_drop(bank_table)?;

            let insert = "INSERT INTO bank (
                bank_name,
                bank_ifsc
            ) VALUES (
                :bank_name,
                :bank_ifsc
            );";

            conn.exec_drop(
                insert,
                params! {
                    "bank_name" => bank.bank_name,
                    "bank_ifsc" => bank.bank_ifsc
                },
            )?;

            Ok(())
        }
    }
}
