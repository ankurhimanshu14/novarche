pub mod roles {

    use cursive::{
        Cursive,
        align::{ HAlign, VAlign },
        view::{ Nameable, Resizable },
        views::{ Dialog, EditView, ListView, SelectView },
    };


    use crate::apis::{
        admin::{
            roles::roles::Roles
        }
    };

    pub fn create_roles(s: &mut Cursive) {
        s.add_layer(
            Dialog::new()
                .title("Create Roles")
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    ListView::new()
                        .child("Roles Name", EditView::new().with_name("roles_name").fixed_width(30))
                )
                .button(
                    "Add",
                    |s| {
                        let roles_name = s.call_on_name("roles_name", |v: &mut EditView| {
                            v.get_content()
                        }).unwrap();
    
                        let new_role = Roles::new(roles_name.to_string());
                        
                        match Roles::post(new_role) {
                            Ok(_) =>{
                                s.pop_layer();
                                s.add_layer(Dialog::text("Role added successfully").dismiss_button("Ok"))
                            },
                            Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                        };
                    }
                )
                .dismiss_button("Cancel")
        )
    }

    pub fn delete_roles(s: &mut Cursive) {
        
        let r = Roles::get().unwrap();

        match r.len() {
            0 => {
                s.add_layer(Dialog::text("No roles defined")
                .button(
                    "Add Roles",
                    |s| { create_roles(s) }
                )
                .dismiss_button("Cancel"))
            },
            _ => {
                s.add_layer(
                    Dialog::new()
                        .title("Delete Roles")
                        .padding_lrtb(1, 1, 1, 0)
                        .content(
                            ListView::new()
                                .child(
                                    "Role Name",
                                    SelectView::<String>::new()
                                        .popup()
                                        .v_align(VAlign::Center)
                                        .autojump()
                                        .with_all_str(r)
                                        .on_select(|s, item| {
                                            println!("{}", &item);
                                        }
                                        )
                                        .with_name("role_name")
                                        .fixed_width(30)
                                )
                            )
                            .button(
                                "Confirm",
                                |s| {
                                    s.pop_layer();
                                    let role_name = s.call_on_name("role_name", |v: &mut SelectView| {
                                        v.selection()
                                    }).unwrap();
        
                                    match Roles::delete(role_name.unwrap().to_string()) {
                                        Ok(_) => {
                                            s.pop_layer();
                                            s.add_layer(Dialog::text("Role name deleted successfully").dismiss_button("Ok"))
                                        },
                                        Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                                    }
                                }
                            )
                            .dismiss_button("Cancel")
                )
            }
        }
    }
}

