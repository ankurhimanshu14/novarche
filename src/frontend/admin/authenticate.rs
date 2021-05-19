pub mod authenticate {

    use cursive::{
        Cursive,
        event::Key,
        align::{ VAlign, HAlign },
        view::{ Nameable, Resizable },
        views::{ Dialog, EditView, ListView, SelectView },
    };

    use crate::apis::admin::authenticate::authenticate::verify_user;

    pub fn sign_in(s: &mut Cursive) {
        s.add_layer(
            Dialog::new()
            .title("Sign In")
            .padding_lrtb(5, 5, 5, 5)
            .content(
                ListView::new()
                .child("Username", EditView::new().with_name("username").fixed_width(30))
                .child("Password", EditView::new().secret().with_name("client_password").fixed_width(30))
            )
            .button(
                "Sign In",
                |s| {
                    let username = s.call_on_name("username", |v:&mut EditView| {
                        v.get_content()
                    }).unwrap();
    
                    let client_password = s.call_on_name("client_password", |v:&mut EditView| {
                        v.get_content()
                    }).unwrap();
    
                    match username.is_empty() || client_password.is_empty() {
                        true => s.add_layer(Dialog::info("All fields must be entered")),
                        false => {
    
                            let ver_user = verify_user(username.to_string(), client_password.to_string()).unwrap();

                            match ver_user {
                                true => {
                                    s.pop_layer();
    
                                    s.add_global_callback(Key::Esc, |s| s.select_menubar());
                    
                                    s.add_layer(Dialog::text("Hit <Esc> to show the menu!"));
                                },
                                false => {
                                    s.add_layer(Dialog::info("Wrong username or password!"));
                                }
                            }

                        }
                    }
                }
            )
            .dismiss_button("Cancel")
        );
    }
}