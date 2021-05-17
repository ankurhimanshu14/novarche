pub mod employee {
    use crate::apis::human_resources::person::person::Person;
    use chrono::NaiveDate;
    use mysql::prelude::*;
    use mysql::*;
    // use crate::utils::parse::parse::parse_from_row;

    #[derive(Debug, Clone)]
    pub struct Employee {
        pub employee_id: String,
        pub dept_code: String,
        pub designation: String,
        pub reporting_to: String,
        pub current_status: String,
        pub date_of_joining: NaiveDate,
        pub date_of_leaving: Option<NaiveDate>,
    }

    impl Employee {
        pub fn new(
            employee_id: String,
            dept_code: String,
            designation: String,
            reporting_to: String,
            current_status: String,
            date_of_joining: NaiveDate,
            date_of_leaving: Option<NaiveDate>,
        ) -> Self {

            Employee {
                employee_id,
                dept_code,
                designation,
                reporting_to,
                current_status,
                date_of_joining,
                date_of_leaving: None
            }
        }

        pub fn post(self, person_id: usize) -> Result<()> {

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
                CONSTRAINT sr_fk_emp_per    FOREIGN KEY(person_id)      REFERENCES          person(uidai) ON DELETE CASCADE ON UPDATE CASCADE,
                CONSTRAINT sr_fk_emp_rep    FOREIGN KEY(reporting_to)   REFERENCES          employee(employee_id) ON DELETE CASCADE ON UPDATE CASCADE,
                CONSTRAINT sr_fk_emp_dept   FOREIGN KEY (dept_code)     REFERENCES          department(department_code) ON DELETE CASCADE ON UPDATE CASCADE
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

            conn.exec_drop(
                insert,
                params! {
                    "employee_id" => self.employee_id,
                    "person_id" => person_id,
                    "dept_code" => self.dept_code,
                    "designation" => self.designation,
                    "reporting_to" => self.reporting_to,
                    "current_status" => self.current_status,
                    "date_of_joining" => self.date_of_joining,
                    "date_of_leaving" => self.date_of_leaving
                },
            )?;

            Ok(())
        }

        pub fn get() -> Result<Vec<Employee>> {
            let query = "SELECT employee_id, dept_code, designation, reporting_to, current_status, date_of_joining, date_of_leaving FROM employee;";

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            let mut v: Vec<Employee> = Vec::new();

            let if_exist = "SELECT COUNT(*)
                FROM information_schema.tables 
                WHERE table_schema = DATABASE()
                AND table_name = 'employee';";

            let result = conn.query_map(
                if_exist,
                |count: usize| {
                    count
                }
            ).unwrap();

            match &result[0] {
                0 => vec![()],
                _ => {
                    conn.query_map(
                        query,
                        |(employee_id, dept_code, designation, reporting_to, current_status, date_of_joining, date_of_leaving)| {
                            let emp = Employee::new(employee_id, dept_code, designation, reporting_to, current_status, date_of_joining, date_of_leaving);
                            v.push(emp);
                        }
                    ).unwrap()
                }
            };
            
            Ok(v)
        }

        pub fn get_employee_id() -> Result<Vec<String>> {
            let mut v: Vec<String> = Vec::new();

            let emp_vec: Vec<Employee> = Employee::get().unwrap();

            for e in emp_vec {
                v.push(e.employee_id)
            }

            Ok(v)
        }
    }
}
