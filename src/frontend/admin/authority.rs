pub mod authority {

    use cursive::{
        Cursive,
        align::{ VAlign, HAlign },
        view::{ Nameable, Resizable },
        views::{ Dialog, EditView, ListView, SelectView },
    };


    use crate::apis::{
        admin::{
            roles::roles::Roles,
            authorities::authorities::Authorities
        }
    };

    pub fn create_authority(s: &mut Cursive) {
        s.add_layer(
            Dialog::new()
                .title("Create Authorities")
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    ListView::new()
                        .child("Activity Name", EditView::new().with_name("activity").fixed_width(30))
                )
                .button(
                    "Add",
                    |s| {

                        let activity = s.call_on_name("activity", |v: &mut EditView| {
                            v.get_content()
                        }).unwrap();

                        let new_authority = Authorities::new(activity.to_string());

                        match Authorities::post(new_authority) {
                            Ok(_) => s.add_layer(Dialog::text("Authority added successfully").dismiss_button("Ok")),
                            Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                        };
                    }
                )
                .dismiss_button("Cancel")
        )
    }

    pub fn assign_authority(s: &mut Cursive) {
        let v = Authorities::get().unwrap();

        let r = Roles::get().unwrap();

        match &v.is_empty() {
            true => {
                s.add_layer(Dialog::info("No activity created!"))
            },
            false => {
                s.add_layer(
                    Dialog::new()
                        .title("Assign Activities")
                        .padding_lrtb(1, 1, 1, 1)
                        .content(
                            ListView::new()
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
                                .child(
                                    "Activity",
                                    SelectView::<String>::new()
                                    .popup()
                                    .h_align(HAlign::Center)
                                    .autojump()
                                    .with_all_str(v)
                                    .on_select(|s, item| {
                                        println!("{}", &item);
                                    }
                                    )
                                    .with_name("activity")
                                    .fixed_width(30)
                                )
                        )
                        .button(
                            "Add",
                            |s| {
                                let rol = s.call_on_name("role", |v: &mut SelectView| {
                                    v.selection()
                                }).unwrap();

                                let sel = s.call_on_name("activity", |v: &mut SelectView| {
                                    v.selection()
                                }).unwrap();

                                match Authorities::assign(rol.unwrap().to_string(), sel.unwrap().to_string()) {
                                    Ok(_) => s.add_layer(Dialog::text("Activity assigned successfully").dismiss_button("Ok")),
                                    Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                                }
                            }
                        ).dismiss_button("Cancel")
                )
            }
        }
    }
}