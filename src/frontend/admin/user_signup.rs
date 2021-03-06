pub mod user_signup {
    
    use cursive::{
        Cursive,
        align::VAlign,
        view::{ Nameable, Resizable },
        views::{ Dialog, EditView, ListView, SelectView },
    };


    use crate::apis::{
        admin::{
            roles::roles::Roles,
            user_signup::user_signup::User,
            authenticate::authenticate::{ get_user, verify_user }
        },
        human_resources::employee::employee::Employee,
    };

    pub fn create_user(s: &mut Cursive) {

        let r = Roles::get().unwrap();

        s.add_layer(
            Dialog::new()
                .title("Create User")
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    ListView::new()
                        .child("Employee ID", EditView::new().with_name("employee_id").fixed_width(30))
                        .child("Username", EditView::new().with_name("username").fixed_width(30))
                        .child("Password", EditView::new().secret().with_name("password").fixed_width(30))
                        .child(
                            "Role",
                            SelectView::<String>::new()
                            .popup()
                            .v_align(VAlign::Center)
                            .autojump()
                            .with_all_str(r)
                            .on_select(|_, item| {
                                println!("{}", &item);
                            }
                            )
                            .with_name("role")
                            .fixed_width(30)
                        )
                )
                .button(
                    "Register",
                    |s| {

                        let employee_id = s.call_on_name("employee_id", |v: &mut EditView| {
                            v.get_content()
                        }).unwrap();

                        let username = s.call_on_name("username", |v: &mut EditView| {
                            v.get_content()
                        }).unwrap();

                        let password = s.call_on_name("password", |v: &mut EditView| {
                            v.get_content()
                        }).unwrap();

                        let role = s.call_on_name("role", |v: &mut SelectView| {
                            v.selection()
                        }).unwrap();

                        let new_user = User::new(
                            employee_id.to_string(),
                            username.to_string(),
                            password.to_string(),
                            role.unwrap().to_string()
                        );

                        match Employee::get_employee(&new_user.employee_id).is_empty() {
                            true => { s.add_layer(Dialog::info("Employee not registered in the system. Contact HR.")) },
                            false => {
                                match User::sign_up(new_user) {
                                    Ok(insert_id) => {
                                        s.pop_layer();
                                        s.add_layer(Dialog::info(format!("User added successfully. Insert ID: {}", insert_id)))
                                    },
                                    Err(e) => s.add_layer(Dialog::info(format!("Sign Up Error encountered: {}", e)))
                                };
                            }
                        }
                    }
                )
                .dismiss_button("Cancel")
        )
    }

    pub fn change_password(s: &mut Cursive) {
        s.add_layer(
            Dialog::new()
                .title("Change Password")
                .padding_lrtb(10, 10, 10, 10)
                .content(
                    ListView::new()
                    .child("Username", EditView::new().with_name("usrnm").fixed_width(30))
                    .child("Old Password", EditView::new().secret().with_name("old_pwd").fixed_width(30))
                    .child("New Password", EditView::new().secret().with_name("new_pwd").fixed_width(30))
                )
                .button(
                    "Update",
                    |s| {
                        let usrnm = s.call_on_name("usrnm", |v: &mut EditView| {
                            v.get_content()
                        }).unwrap();

                        let old_pwd = s.call_on_name("old_pwd", |v: &mut EditView| {
                            v.get_content()
                        }).unwrap();

                        let new_pwd = s.call_on_name("new_pwd", |v: &mut EditView| {
                            v.get_content()
                        }).unwrap();

                        let user = get_user(usrnm.to_string()).unwrap();

                        match verify_user(user[0].clone(), old_pwd.to_string()) {
                            true => {
                                match User::change_password(new_pwd.to_string(), usrnm.to_string()) {
                                    Ok(_) => {
                                        s.pop_layer();
                                        s.add_layer(Dialog::text("Password changed successfully").dismiss_button("Ok"))
                                    },
                                    Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                                }
                            },
                            false => {
                                s.pop_layer();
                                s.add_layer(Dialog::text("Wrong Credentials").dismiss_button("Ok"))
                            }
                        }
                    }
                )
                .dismiss_button("Cancel")
        )
    }
}