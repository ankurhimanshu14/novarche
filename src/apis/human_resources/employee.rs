pub mod employee {
    use chrono::{ DateTime, Utc, NaiveDate };
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug, Clone)]
    pub struct Employee {
        pub employee_id: usize,
        pub first_name: String,
        pub middle_name: Option<String>,
        pub last_name: String,
        pub gender: String,
        pub father_name: String,
        pub mother_name: String,
        pub spouse_name: Option<String>,
        bank_name: Option<String>,
        bank_ifsc: Option<String>,
        bank_acc_no: Option<usize>,
        pub date_of_birth: NaiveDate,
        pub pan: Option<String>,
        pub uidai: Option<usize>,
        pub uan: Option<usize>,
        pub dept: String,
        pub designation: String,
        pub reporting_to: i32,
        pub current_status: String,
        pub date_of_joining: NaiveDate,
        pub date_of_leaving: Option<NaiveDate>,
        pub created_at: DateTime<Utc>,
        pub modified_at: Option<DateTime<Utc>>
    }

    impl Employee {
        pub fn new() -> Self {
            let mut employee_id = String::new();
            println!("Enter EmployeeId:");
            std::io::stdin()
                            .read_line(&mut employee_id)
                            .expect("Failed to read input");
            
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

            middle_name = match middle_name {
                v => v,
                _ => "".to_string()
            };

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

            let mut father_name = String::new();
            println!("Enter Father's Name:");
            std::io::stdin()
                            .read_line(&mut father_name)
                            .expect("Failed to read input");

            let mut mother_name = String::new();
            println!("Enter Mother's Name:");
            std::io::stdin()
                            .read_line(&mut mother_name)
                            .expect("Failed to read input");

            let mut spouse_name = String::new();
            println!("Enter Spouse's Name:");
            std::io::stdin()
                            .read_line(&mut spouse_name)
                            .expect("Failed to read input");

            spouse_name = match spouse_name {
                v => v,
                _ => "".to_string()
            };

            let mut bank_name = String::new();
            println!("Enter Bank's Name:");
            std::io::stdin()
                            .read_line(&mut bank_name)
                            .expect("Failed to read input");

            bank_name = match bank_name {
                v => v,
                _ => "".to_string()
            };

            let mut bank_ifsc = String::new();
            println!("Enter IFSC:");
            std::io::stdin()
                            .read_line(&mut bank_ifsc)
                            .expect("Failed to read input");

            bank_ifsc = match bank_ifsc {
                v => v,
                _ => "".to_string()
            };

            let mut bank_acc_no = String::new();
            println!("Enter Bank Account No.:");
            std::io::stdin()
                            .read_line(&mut bank_acc_no)
                            .expect("Failed to read input");

            bank_acc_no = match bank_acc_no {
                v => v,
                _ => "".to_string()
            };

            let mut dob = String::new();
            println!("Enter date of birth (dd-mm--yyyy):");
            std::io::stdin()
                            .read_line(&mut dob)
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
            
            let mut dept = String::new();
            println!("Enter Department:");
            std::io::stdin()
                            .read_line(&mut dept)
                            .expect("Failed to read input");
            
            let mut designation = String::new();
            println!("Enter Designation:");
            std::io::stdin()
                            .read_line(&mut designation)
                            .expect("Failed to read input");
            
            let mut reporting_to = String::new();
            println!("Enter Reproting to:");
            std::io::stdin()
                            .read_line(&mut reporting_to)
                            .expect("Failed to read input");

            let mut current_status = String::new();
            println!("Enter Status:");
            std::io::stdin()
                            .read_line(&mut current_status)
                            .expect("Failed to read input");

            let mut doj = String::new();
            println!("Enter date of joining (dd-mm--yyyy):");
            std::io::stdin()
                            .read_line(&mut doj)
                            .expect("Failed to read input");

            let mut dol = String::new();
            println!("Enter date of leaving (dd-mm--yyyy):");
            std::io::stdin()
                            .read_line(&mut dol)
                            .expect("Failed to read input");

            
            Employee {
                employee_id: employee_id.parse().unwrap_or(0),
                
                first_name: first_name.to_string()
                                .trim_end_matches("\r\n")
                                .to_string(),
                middle_name: Some(middle_name.to_string()
                                .trim_end_matches("\r\n")
                                .to_string()),

                last_name: last_name.to_string()
                                .trim_end_matches("\r\n")
                                .to_string(),

                gender: gender.to_string()
                                .trim_end_matches("\r\n")
                                .to_string(),

                father_name: father_name.to_string()
                                .trim_end_matches("\r\n")
                                .to_string(),

                mother_name: mother_name.to_string()
                                .trim_end_matches("\r\n")
                                .to_string(),

                spouse_name: match spouse_name.to_string()
                                .trim_end_matches("\r\n")
                                .to_string() {
                                    d => Some(d),
                                    _ => None
                                },

                bank_name: match bank_name.to_string()
                                .trim_end_matches("\r\n")
                                .to_string() {
                                    d => Some(d),
                                    _ => None
                                },

                bank_ifsc: match bank_ifsc.to_string()
                                .trim_end_matches("\r\n")
                                .to_string() {
                                    d => Some(d),
                                    _ => None
                                },

                bank_acc_no: match bank_acc_no.to_string()
                                .trim_end_matches("\r\n")
                                .to_string() {
                                    d => Some(d.parse::<usize>().unwrap()),
                                    _ => None
                                },

                date_of_birth: NaiveDate::parse_from_str(&dob, "%d-%m-%Y").unwrap(),

                pan: match pan.to_string()
                        .trim_end_matches("\r\n")
                        .to_string() {
                            d => Some(d),
                            _ => None
                        },

                uidai: match uidai.parse().unwrap_or(0) {
                    d => Some(d),
                    _ => None
                },

                uan: match uan.parse().unwrap_or(0) {
                    d => Some(d),
                    _ => None
                },

                dept: dept.to_string()
                                .trim_end_matches("\r\n")
                                .to_string(),

                designation: designation.to_string()
                                .trim_end_matches("\r\n")
                                .to_string(),
                reporting_to: reporting_to.parse().unwrap_or(0),

                current_status: current_status.to_string()
                        .trim_end_matches("\r\n")
                        .to_string(),

                date_of_joining: NaiveDate::parse_from_str(&doj, "%d-%m-%Y").unwrap(),
                
                date_of_leaving: match NaiveDate::parse_from_str(&dol, "%d-%m-%Y").unwrap() {
                    d => Some(d),
                    _ => None
                },

                created_at: Utc::now(),

                modified_at: None
            }
        }

        pub fn post(e: Employee) -> Result<()> {
            let table = "CREATE TABLE IF NOT EXISTS employee(
                id                  INT             NOT NULL            PRIMARY KEY         AUTO_INCREMENT,
                employee_id         INT             NOT NULL            UNIQUE,
                first_name          VARCHAR(100)    NOT NULL,
                middle_name         VARCHAR(100),
                last_name           VARCHAR(100)    NOT NULL,
                gender              VARCHAR(100)    NOT NULL,
                father_name         VARCHAR(100)    NOT NULL,
                mother_name         VARCHAR(100)    NOT NULL,
                spouse_name         VARCHAR(100),
                bank_name           VARCHAR(100),
                bank_ifsc           VARCHAR(100),
                bank_acc_no         INT,
                pan                 VARCHAR(10),
                uidai               INT,
                uan                 INT,
                dept                VARCHAR(20),
                designation         VARCHAR(30),
                reporting_to        INT,
                current_status      VARCHAR(10),
                CONSTRAINT sr_fk_emp_man FOREIGN KEY (reporting_to) REFERENCES employee(employee_id)
            ) ENGINE = InnoDB;";

            let insert = "INSERT INTO employee(
                employee_id,
                first_name,
                middle_name,
                last_name,
                gender,
                father_name,
                mother_name,
                spouse_name,
                bank_name,
                bank_ifsc,
                bank_acc_no,
                pan,
                uidai,
                uan,
                dept,
                designation,
                reporting_to,
                current_status
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;
            conn.exec_drop(insert, (e.employee_id, e.first_name, e.middle_name, e.last_name, e.gender, e.father_name, e.mother_name, e.spouse_name, e.bank_name, e.bank_ifsc, e.bank_acc_no, e.pan, e.uidai))?;

            Ok(())
        }
    }
}