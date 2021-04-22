mod apis;

use crate::apis::admin::user::user::User;
use crate::apis::accounts::bank::bank::Bank;
use crate::apis::human_resources::employee::employee::Employee;
use crate::apis::admin::department::department::Department;

fn main() -> Result<(), mysql::Error> {

    let new_emp = Employee::new();

    Employee::post(new_emp)?;

    Ok(())
}
