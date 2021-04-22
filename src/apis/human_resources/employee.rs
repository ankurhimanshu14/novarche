pub mod employee {
    use chrono::{ NaiveDate };
    use mysql::*;
    use mysql::prelude::*;
    use crate::apis::human_resources::person::person::Person;

    #[derive(Debug, Clone)]
    pub struct Employee {
        pub employee_id: String,
        pub dept_code: String,
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
            
            let mut dept_code = String::new();
            println!("Enter Department:");
            std::io::stdin()
                            .read_line(&mut dept_code)
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

                dept_code: dept_code.to_string()
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

            let new_individual = Person::new();

            Person::post(&new_individual)?;

            let table = r"CREATE TABLE IF NOT EXISTS employee(
                id                  INT             NOT NULL            PRIMARY KEY         AUTO_INCREMENT,
                employee_id         VARCHAR(10)     NOT NULL            UNIQUE,
                person_id           BIGINT          NOT NULL,
                dept_code           VARCHAR(4)      NOT NULL,
                designation         VARCHAR(50)     NOT NULL,
                reporting_to        VARCHAR(10)     NOT NULL,
                current_status      VARCHAR(10)     NOT NULL,
                date_of_joining     DATETIME        NOT NULL,
                date_of_leaving     DATETIME,
                created_at          DATETIME        NOT NULL            DEFAULT             CURRENT_TIMESTAMP,
                modified_at         DATETIME                            ON UPDATE           CURRENT_TIMESTAMP,
                CONSTRAINT sr_fk_emp_per    FOREIGN KEY(person_id)      REFERENCES          person(uidai),
                CONSTRAINT sr_fk_emp_rep    FOREIGN KEY(reporting_to)   REFERENCES          employee(employee_id),
                CONSTRAINT sr_fk_emp_dept   FOREIGN KEY (dept_code)     REFERENCES          department(department_code)        
            ) ENGINE = InnoDB;";

            let insert = r"INSERT INTO employee(
                employee_id,
                person_id,
                dept_code,
                designation,
                reporting_to,
                current_status,
                date_of_joining,
                date_of_leaving
            ) VALUES (
                :employee_id,
                :person_id,
                :dept_code,
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
                "person_id" => new_individual.uidai,
                "dept_code" => e.dept_code,
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