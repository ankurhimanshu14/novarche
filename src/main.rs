mod apis;

use crate::apis::admin::user::user::User;

fn main() {
    let new_user = User::new();
    User::post(new_user);
    User::get();
}
