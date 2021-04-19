mod apis;

use crate::apis::admin::user::user::User;

fn main() {
    // let new_user = User::new();
    // User::push(new_user);
    User::get();

    // println!("{:#?}", all_users);
}