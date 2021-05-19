pub mod department {

    use cursive::{
        Cursive,
        align::{ HAlign, VAlign },
        view::{ Nameable, Resizable },
        views::{ Dialog, EditView, ListView, SelectView },
    };


    use crate::apis::{
        human_resources::{
            department::department::Department
        }
    };

    pub fn create_department(s: &mut Cursive) {
        s.add_layer(
            Dialog::new()
            .title("Create Department")
            .padding_lrtb(1, 1, 1, 0)
            .content(
                ListView::new()
                    .child("Department Code", EditView::new().with_name("department_code").fixed_width(30))
                    .child("Description", EditView::new().with_name("description").fixed_width(30))
                    .child("email", EditView::new().with_name("email").fixed_width(30))
            )
            .button(
                "Add",
                |s| {
                    let department_code = s.call_on_name("department_code", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();
                    let description = s.call_on_name("description", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();
                    let email = s.call_on_name("email", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let new_department = Department::new(department_code.to_string(), description.to_string(), email.to_string());

                    match Department::post(new_department) {
                        Ok(_) => {
                            s.pop_layer();
                            s.add_layer(Dialog::text("Department added successfully").dismiss_button("Ok"))
                        },
                        Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                    };
                }
            )
            .dismiss_button("Cancel")
        );
    }
}