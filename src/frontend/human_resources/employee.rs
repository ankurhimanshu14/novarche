pub mod employee {
    use std::panic;
    use chrono::NaiveDate;
    
    use cursive::{
        Cursive,
        align::{ HAlign, VAlign },
        view::{ Nameable, Resizable },
        views::{ Dialog, EditView, ListView, SelectView },
    };

    use crate::apis::{
        human_resources::{
            department::department::Department,
            person::person::Person,
            employee::employee::Employee
        }
    };

    use crate::frontend::{
        human_resources::department::department::create_department,
    };

    pub fn create_employee(s: &mut Cursive) {

        let dept = Department::get_dept_code_list().unwrap();

        let rep_to = Employee::get_employee_id_list().unwrap();

        match dept.is_empty() {
            true => {
                s.add_layer(Dialog::text("No department created!")
                .button(
                    "Create Department",
                    |s| { create_department(s) }
                )
                .dismiss_button("Cancel"))
            },
            false => s.add_layer(
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
                            .child("UIDAI *", EditView::new().with_name("uidai").fixed_width(30))
                            .child("UAN", EditView::new().with_name("uan").fixed_width(30))
                            .child("Employee ID *", EditView::new().with_name("employee_id").fixed_width(30))
                            .child(
                                "Department *",
                                SelectView::<String>::new()
                                .popup()
                                .h_align(HAlign::Center)
                                .autojump()
                                .with_all_str(dept)
                                .on_select(|_, item| {
                                    println!("{}", &item);
                                })
                                .with_name("department_code")
                                .fixed_width(30)
                            )
                            .child("Designation *", EditView::new().with_name("designation").fixed_width(30))
                            .child(
                                "Reporting To *",
                                SelectView::<String>::new()
                                .popup()
                                .h_align(HAlign::Center)
                                .autojump()
                                .with_all_str(rep_to)
                                .on_select(|_, item| {
                                    println!("{}", &item);
                                })
                                .with_name("reporting_to")
                                .fixed_width(30)
                            )
                            .child(
                                "Current Status *",
                                SelectView::<String>::new()
                                .popup()
                                .v_align(VAlign::Center)
                                .autojump()
                                .with_all_str(vec!["EMPLOYED", "RETIRED"])
                                .on_select(|_, item| {
                                    println!("{}", &item);
                                })
                                .with_name("current_status")
                                .fixed_width(30)
                            )
                            .child("Date of joining *", EditView::new().with_name("date_of_joining").fixed_width(30))
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
    
                            let date_of_birth = NaiveDate::parse_from_str(&dob.to_string(), "%d-%m-%Y").unwrap();
    
                            let pri_contact_no = s.call_on_name("pri_contact_no", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
                            
                            let sec_contact_no = s.call_on_name("sec_contact_no", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
    
                            let personal_email = s.call_on_name("personal_email", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
    
                            let per_address = s.call_on_name("per_address", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
    
                            let com_address = s.call_on_name("com_address", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
    
                            let pan = s.call_on_name("pan", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
    
                            let uidai = s.call_on_name("uidai", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
    
                            let uan = s.call_on_name("uan", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
    
                            let uan = match &uan.len() {
                                0 => None,
                                _ => Some(uan.to_string().parse::<usize>().unwrap())
                            };

                            let employee_id = s.call_on_name("employee_id", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();

                            let department_code = s.call_on_name("department_code", |v: &mut SelectView| {
                                v.selection()
                            }).unwrap();

                            let designation = s.call_on_name("designation", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();

                            let reporting_to = s.call_on_name("reporting_to", |v: &mut SelectView| {
                                v.selection()
                            }).unwrap();

                            let reporting_to = match reporting_to {
                                Some(reporting_to) => reporting_to,
                                None => employee_id.clone()
                            };

                            let current_status = s.call_on_name("current_status", |v: &mut SelectView| {
                                v.selection()
                            }).unwrap();

                            let doj = s.call_on_name("date_of_joining", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();

                            let date_of_joining = NaiveDate::parse_from_str(&doj.to_string(), "%d-%m-%Y").unwrap();

                            let date_of_leaving = None;

    
                            match first_name.clone().is_empty() || last_name.clone().is_empty() || gender.clone().unwrap().is_empty() || dob.clone().is_empty() || per_address.clone().is_empty() || pan.clone().to_string().is_empty() || uidai.clone().to_string().is_empty() {
                                true => { s.add_layer(Dialog::info("* marked are required fields")) },
                                false => {
                                    match Person::new(
                                        first_name.to_string(),
                                        Some(middle_name.to_string()),
                                        last_name.to_string(),
                                        gender.clone().unwrap().to_string(),
                                        date_of_birth,
                                        Some(pri_contact_no.to_string()),
                                        Some(sec_contact_no.to_string()),
                                        Some(personal_email.to_string()),
                                        per_address.to_string(),
                                        Some(com_address.to_string()),
                                        pan.to_string(),
                                        uidai.parse::<usize>().unwrap(),
                                        uan
                                    ).post() {
                                        Ok(_) => {
                                            s.add_layer(Dialog::text("Person added successfully").dismiss_button("Ok"))
                                        },
                                        Err(e) => s.add_layer(Dialog::info(format!("Error encountered: {}", e)))
                                    };

                                    let new_emp = Employee::new(
                                        employee_id.to_string(),
                                        department_code.clone().unwrap().to_string(),
                                        designation.to_string(),
                                        reporting_to.to_string(),
                                        current_status.clone().unwrap().to_string(),
                                        date_of_joining,
                                        date_of_leaving
                                    );
                                    
                                    match Employee::post(new_emp, uidai.clone().parse::<usize>().unwrap()) {
                                        Ok(_) => {
                                            s.pop_layer();
                                            s.add_layer(Dialog::text("Employee added successfully").dismiss_button("Ok"))
                                        },
                                        Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                                    };
                                }
                            }
                        }
                    )
                    .dismiss_button("Cancel")
            )
        }
    }
}