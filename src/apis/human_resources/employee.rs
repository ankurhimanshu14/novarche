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
                date_of_leaving
                
                // : NaiveDate::parse_from_str(
                //     &doj.to_string().trim_end_matches("\r\n").to_string(),
                //     "%d-%m-%Y",
                // )
                // .unwrap(),
                // date_of_leaving: match &dol.to_string().trim_end_matches("\r\n").to_string().len() {
                //     0 => None,
                //     _ => Some(
                //         NaiveDate::parse_from_str(
                //             &dol.to_string().trim_end_matches("\r\n").to_string(),
                //             "%d-%m-%Y",
                //         )
                //         .unwrap(),
                //     ),
                // },
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
                    "employee_id" => e.employee_id,
                    "person_id" => new_individual.uidai,
                    "dept_code" => e.dept_code,
                    "designation" => e.designation,
                    "reporting_to" => e.reporting_to,
                    "current_status" => e.current_status,
                    "date_of_joining" => e.date_of_joining,
                    "date_of_leaving" => e.date_of_leaving
                },
            )?;

            Ok(())
        }

        pub fn get() -> Result<Vec<Vec<String>>> {
            let query = format!(
                "SELECT e.employee_id, p.first_name, p.middle_name, p.last_name, e.dept_code, p.uan, e. designation, e.reporting_to
                FROM employee e
                LEFT JOIN person p
                ON p.uidai = e.person_id;");

            let url = "mysql://root:@localhost:3306/mws_database".to_string();

            let pool = Pool::new(url)?;

            let conn = pool.get_conn()?;

            let result: Vec<Row> = query.fetch(conn)?;

            let mut v1: Vec<Vec<String>> = Vec::new();

            for entries in result.iter() {
                let length: &usize = &entries.len();
                let mut v2: Vec<String> = Vec::new();
                for index in 0..*length {
                    let val = &entries.get_opt::<String, usize>(index).unwrap();

                    match val {
                        Ok(_) => v2.push(val.as_ref().unwrap().to_string()),
                        Err(_) => v2.push("".to_string()),
                    }
                }
                v1.push(v2);
            }
            Ok(v1)
        }
    }
}
