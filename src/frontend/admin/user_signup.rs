pub mod user_signup {
    
    use cursive::{
        Cursive,
        align::{ HAlign, VAlign },
        view::{ Nameable, Resizable },
        views::{ Dialog, EditView, ListView, SelectView },
    };


    use crate::apis::{
        admin::{
            roles::roles::Roles,
            user_signup::user_signup::User,
        },
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
                            .on_select(|s, item| {
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

                        match User::sign_up(new_user) {
                            Ok(_) => s.add_layer(Dialog::text("User added successfully").dismiss_button("Ok")),
                            Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                        };
                    }
                )
                .dismiss_button("Cancel")
        )
    }
}