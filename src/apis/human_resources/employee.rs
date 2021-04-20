pub mod employee {
    use chrono::{ NaiveDate };
    use mysql::*;
    use mysql::prelude::*;

    #[derive(Debug, Clone)]
    pub struct Employee {
        pub employee_id: String,
        pub first_name: String,
        pub middle_name: Option<String>,
        pub last_name: String,
        pub gender: String,
        father_name: String,
        mother_name: String,
        spouse_name: Option<String>,
        bank_name: Option<String>,
        bank_ifsc: Option<String>,
        bank_acc_no: Option<usize>,
        basic_salary: usize,
        date_of_birth: NaiveDate,
        pan: Option<String>,
        uidai: Option<usize>,
        uan: Option<usize>,
        pub dept: String,
        pub designation: String,
        pub reporting_to: String,
        pub current_status: String,
        pub date_of_joining: NaiveDate,
        pub date_of_leaving: Option<NaiveDate>
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

            let mut bank_name = String::new();
            println!("Enter Bank's Name:");
            std::io::stdin()
                            .read_line(&mut bank_name)
                            .expect("Failed to read input");

            let mut bank_ifsc = String::new();
            println!("Enter IFSC:");
            std::io::stdin()
                            .read_line(&mut bank_ifsc)
                            .expect("Failed to read input");

            let mut bank_acc_no = String::new();
            println!("Enter Bank Account No.:");
            std::io::stdin()
                            .read_line(&mut bank_acc_no)
                            .expect("Failed to read input");

            let mut basic_salary = String::new();
            println!("Enter Basic Salary.:");
            std::io::stdin()
                            .read_line(&mut basic_salary)
                            .expect("Failed to read input");

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
            println!("Enter Reporting to:");
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
                employee_id: employee_id.to_string()
                                .trim_end_matches("\r\n")
                                .to_string(),
                
                first_name: first_name.to_string()
                                .trim_end_matches("\r\n")
                                .to_string(),
                middle_name: match middle_name.to_string().trim_end_matches("\r\n").to_string().len() {
                    0 => None,
                    _ => Some(middle_name.to_string().trim_end_matches("\r\n").to_string())
                },

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

                spouse_name: match spouse_name.to_string().trim_end_matches("\r\n").to_string().len() {
                    0 => None,
                    _ => Some(spouse_name.to_string().trim_end_matches("\r\n").to_string())
                },

                bank_name: match bank_name.to_string().trim_end_matches("\r\n").to_string().len() {
                    0 => None,
                    _ => Some(bank_name.to_string().trim_end_matches("\r\n").to_string())
                },

                bank_ifsc: match bank_ifsc.to_string().trim_end_matches("\r\n").to_string().len() {
                    0 => None,
                    _ => Some(bank_ifsc.to_string().trim_end_matches("\r\n").to_string())
                },

                bank_acc_no: match bank_acc_no.to_string().trim_end_matches("\r\n").to_string().len() {
                    0 => None,
                    _ => Some(bank_acc_no.parse().unwrap_or(0))
                },

                basic_salary: basic_salary.parse().unwrap_or(0),

                date_of_birth: NaiveDate::parse_from_str(&dob.to_string().trim_end_matches("\r\n").to_string(), "%d-%m-%Y").unwrap(),

                pan: match pan.to_string().trim_end_matches("\r\n").to_string().len() {
                    0 => None,
                    _ => Some(pan.to_string().trim_end_matches("\r\n").to_string())
                },

                uidai: match uidai.to_string().trim_end_matches("\r\n").to_string().len() {
                    0 => None,
                    _ => Some(uidai.parse().unwrap_or(0))
                },

                uan: match uan.to_string().trim_end_matches("\r\n").to_string().len() {
                    0 => None,
                    _ => Some(uan.parse().unwrap_or(0))
                },

                dept: dept.to_string()
                                .trim_end_matches("\r\n")
                                .to_string(),

                designation: designation.to_string()
                                .trim_end_matches("\r\n")
                                .to_string(),

                reporting_to: reporting_to.to_string()
                                .trim_end_matches("\r\n")
                                .to_string(),

                current_status: current_status.to_string()
                        .trim_end_matches("\r\n")
                        .to_string(),

                date_of_joining: NaiveDate::parse_from_str(&doj.to_string().trim_end_matches("\r\n").to_string(), "%d-%m-%Y").unwrap(),
                
                date_of_leaving: match &dol.to_string().trim_end_matches("\r\n").to_string().len() {
                    0 => None,
                    _ => Some(NaiveDate::parse_from_str(&dol.to_string().trim_end_matches("\r\n").to_string(), "%d-%m-%Y").unwrap())
                }
            }
        }

        pub fn post(e: Employee) -> Result<()> {
            let table = r"CREATE TABLE IF NOT EXISTS employee(
                id                  INT             NOT NULL            PRIMARY KEY         AUTO_INCREMENT,
                employee_id         VARCHAR(10)     NOT NULL            UNIQUE,
                first_name          VARCHAR(100)    NOT NULL,
                middle_name         VARCHAR(100),
                last_name           VARCHAR(100)    NOT NULL,
                gender              VARCHAR(100)    NOT NULL,
                date_of_birth       DATETIME        NOT NULL,
                father_name         VARCHAR(100)    NOT NULL,
                mother_name         VARCHAR(100)    NOT NULL,
                spouse_name         VARCHAR(100),
                bank_name           VARCHAR(100),
                bank_ifsc           VARCHAR(100),
                bank_acc_no         INT,
                pan                 VARCHAR(10),
                uidai               INT,
                uan                 INT,
                dept                VARCHAR(20)     NOT NULL,
                designation         VARCHAR(50)     NOT NULL,
                reporting_to        VARCHAR(10)     NOT NULL,
                current_status      VARCHAR(10)     NOT NULL,
                date_of_joining     DATETIME        NOT NULL,
                date_of_leaving     DATETIME,
                created_at          DATETIME        NOT NULL        DEFAULT     CURRENT_TIMESTAMP,
                modified_at         DATETIME                        ON UPDATE   CURRENT_TIMESTAMP,
                CONSTRAINT sr_fk_emp_man FOREIGN KEY (reporting_to) REFERENCES employee(employee_id)           
            ) ENGINE = InnoDB;";

            let insert = r"INSERT INTO employee(
                employee_id,
                first_name,
                middle_name,
                last_name,
                gender,
                date_of_birth,
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
                current_status,
                date_of_joining,
                date_of_leaving
            ) VALUES (
                :employee_id,
                :first_name,
                :middle_name,
                :last_name,
                :gender,
                :date_of_birth,
                :father_name,
                :mother_name,
                :spouse_name,
                :bank_name,
                :bank_ifsc,
                :bank_acc_no,
                :pan,
                :uidai,
                :uan,
                :dept,
                :designation,
                :reporting_to,
                :current_status,
                :date_of_joining,
                :date_of_leaving
            );";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            conn.exec_drop(insert, params! {
                "employee_id" => e.employee_id,
                "first_name" => e.first_name,
                "middle_name" => e.middle_name,
                "last_name" => e.last_name,
                "gender" => e.gender,
                "date_of_birth" => e.date_of_birth,
                "father_name" => e.father_name,
                "mother_name" => e.mother_name,
                "spouse_name" => e.spouse_name.unwrap(),
                "bank_name" => e.bank_name.unwrap(),
                "bank_ifsc" => e.bank_ifsc.unwrap(),
                "bank_acc_no" => e.bank_acc_no.unwrap(),
                "pan" => e.pan.unwrap(),
                "uidai" => e.uidai.unwrap(),
                "uan" => e.uan.unwrap(),
                "dept" => e.dept,
                "designation" => e.designation,
                "reporting_to" => e.reporting_to,
                "current_status" => e.current_status,
                "date_of_joining" => e.date_of_joining,
                "date_of_leaving" => e.date_of_leaving
            })?;

            Ok(())
        }
    }
}