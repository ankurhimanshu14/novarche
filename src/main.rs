mod apis;

use crate::apis::admin::user::user::User;

fn main() -> Result<(), mysql::Error> {
    let new_user = User::new();
    User::post(new_user)?;
    let user = User::get_one("ankur123".to_string()).unwrap();

    println!("{:#?}", &user[0]);

    User::verify("ankur123".to_string(), "Ankur@123".to_string());
    Ok(())
}
