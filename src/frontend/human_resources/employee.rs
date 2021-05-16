pub mod employee {

    use chrono::NaiveDate;
    
    use cursive::{
        Cursive,
        align::{ HAlign, VAlign },
        view::{ Nameable, Resizable },
        views::{ Dialog, EditView, ListView, SelectView },
    };

    use crate::apis::{
        human_resources::{
            person::person::Person,
            employee::employee::Employee
        }
    };

    pub fn create_employee(s: &mut Cursive) {
        s.add_layer(
            Dialog::new()
                .title("New Joining")
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    ListView::new()
                        .child("First Name *", EditView::new().with_name("first_name").fixed_width(30))
                        .child("Middle Name", EditView::new().with_name("middle_name").fixed_width(30))
                        .child("Last Name *", EditView::new().with_name("last_name").fixed_width(30))
                        .child(
                            "Gender *",
                            SelectView::<String>::new()
                            .popup()
                            .v_align(VAlign::Center)
                            .autojump()
                            .with_all_str(vec!["MALE", "FEMALE", "OTHERS"])
                            .on_select(|_, item| {
                                println!("{}", &item);
                            }
                        )
                        .with_name("gender")
                        .fixed_width(30))
                        .child("Date of birth *", EditView::new().with_name("date_of_birth").fixed_width(30))
                        .child("Primary Contact No.", EditView::new().with_name("pri_contact_no").fixed_width(30))
                        .child("Secondary Contact No.", EditView::new().with_name("sec_contact_no").fixed_width(30))
                        .child("Personal Email ID", EditView::new().with_name("personal_email").fixed_width(30))
                        .child("Permanent Address *", EditView::new().with_name("per_address").fixed_width(30))
                        .child("Communication Address", EditView::new().with_name("com_address").fixed_width(30))
                        .child("PAN", EditView::new().with_name("pan").fixed_width(30))
                        .child("UIDAI", EditView::new().with_name("uidai").fixed_width(30))
                        .child("UAN", EditView::new().with_name("uan").fixed_width(30))
                )
                .button(
                    "Add",
                    |s| {
                        let first_name = s.call_on_name("first_name", |v: &mut EditView| {
                            v.get_content()
                        }).unwrap();

                        let middle_name = s.call_on_name("middle_name", |v: &mut EditView| {
                            v.get_content()
                        }).unwrap();

                        let last_name = s.call_on_name("last_name", |v: &mut EditView| {
                            v.get_content()
                        }).unwrap();

                        let gender = s.call_on_name("gender", |v: &mut SelectView| {
                            v.selection()
                        }).unwrap();

                        let dob = s.call_on_name("date_of_birth", |v: &mut EditView| {
                            v.get_content()
                        }).unwrap();

                        let date_of_birth = NaiveDate::parse_from_str(
                            &dob.to_string(),
                            "%d-%m-%Y",
                        ).unwrap();

                        match &first_name.len() | &last_name.len() | &gender.unwrap().len() | &dob.len() {
                            0 => { s.add_layer(Dialog::info("* marked are required fields")) },
                            _ => { }
                        }

                        Person::new(
                            first_name.to_string(),
                            Some(middle_name.to_string()),
                            last_name.to_string(),
                            gender.unwrap().to_string(),
                            date_of_birth,
                            pri_contact_no.to_string(),
                            sec_contact_no.to_string(),
                            personal_email.to_string(),
                            per_address.to_string(),
                            com_address.to_string(),
                            pan.to_string(),
                            uidai.to_string(),
                            uan.to_string()
                        ).post();
                      }
                )
                .dismiss_button("Cancel")
        );
    }
}