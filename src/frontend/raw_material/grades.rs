pub mod grades {

    use cursive::{
        Cursive,
        traits::*,
        CursiveExt,
        event::Key,
        menu,
        view::{ Nameable, Resizable },
        align::{ HAlign, VAlign },
        views::{ Menubar, Dialog, EditView, ListView, SelectView, TextView },
    };

    use crate::apis::raw_material::grades::grades::Grades;

    pub fn create_grades(s: &mut Cursive) {
        s.add_layer(
            Dialog::new()
                .title("Create Grades")
                .padding_lrtb(1, 1, 1, 0)
                .content(
                    ListView::new()
                        .child("Grade Name", EditView::new().with_name("grade_name").fixed_width(30))
                )
                .button(
                    "Add",
                    |s| {
                        let grade_name = s.call_on_name("grade_name", |v: &mut EditView| {
                            v.get_content()
                        }).unwrap();
    
                        let new_grade = Grades::new(grade_name.to_string());
                        
                        match Grades::post(&new_grade) {
                            Ok(_) =>{
                                s.pop_layer();
                                s.add_layer(Dialog::text("Grade added successfully").dismiss_button("Ok"))
                            },
                            Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                        };
                    }
                )
                .dismiss_button("Cancel")
        )
    }
}