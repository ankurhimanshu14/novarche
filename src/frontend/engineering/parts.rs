pub mod parts {

    use cursive::{
        Cursive,
        event::Key,
        align::{ VAlign, HAlign },
        view::{ Nameable, Resizable },
        views::{ Dialog, EditView, ListView, SelectView },
    };

    use crate::apis::{
        raw_material::grades::grades::Grades,
        engineering::part::part::Part
    };

    pub fn create_parts(s: &mut Cursive) {

        let grd = Grades::get_list();

            s.add_layer(
                Dialog::new()
                    .title("Add new part")
                    .padding_lrtb(1,1,1,0)
                    .content(
                        ListView::new()
                            .child("Part No", EditView::new().with_name("part_no").fixed_width(30))
                            .child("Part Name", EditView::new().with_name("part_name").fixed_width(30))
                            .child(
                                "Material Grade",
                                SelectView::new()
                                    .popup()
                                    .v_align(VAlign::Center)
                                    .autojump()
                                    .with_all_str(grd)
                                    .on_select(|s, item| {
                                        println!("{}", &item)
                                    })
                                    .with_name("grade")
                                    .fixed_width(30))
                            .child("Forging Weight", EditView::new().with_name("forging_wt").fixed_width(30))
                            .child("Cut Weight", EditView::new().with_name("cut_wt").fixed_width(30))
                            .child(
                                "Delivery Condition",
                                SelectView::new()
                                .popup()
                                .v_align(VAlign::Center)
                                .autojump()
                                .with_all_str(&["Normalized".to_string(), "Isothermal Annealed".to_string(), "Annealed".to_string(), "As Forged".to_string()])
                                .on_select(|s, item| {
                                    println!("{}", &item)
                                })
                                .with_name("del_cond")
                                .fixed_width(30)
                            )
                            .child("Drawing Revision No", EditView::new().with_name("drawing_rev_no").fixed_width(30))
                            .child("Drawing Revision Date", EditView::new().with_name("drawing_rev_date").fixed_width(30))
                    )
                    .button(
                        "Add",
                        |s| {
                            let part_no = s.call_on_name("part_no", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
                            let part_name = s.call_on_name("part_name", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
                            let grade = s.call_on_name("grade", |v: &mut SelectView| {
                                v.selection()
                            }).unwrap();
                            let forging_wt = s.call_on_name("forging_wt", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
                            let cut_wt = s.call_on_name("cut_wt", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
                            let del_cond = s.call_on_name("del_cond", |v: &mut SelectView| {
                                v.selection()
                            }).unwrap();
                            let drawing_rev_no = s.call_on_name("drawing_rev_no", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
                            let drawing_rev_date = s.call_on_name("drawing_rev_date", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();

                            match Part::new(
                                part_no.parse::<usize>().unwrap(),
                                part_name.to_string(),
                                grade.unwrap().to_string(),
                                forging_wt.parse::<f32>().unwrap(),
                                cut_wt.parse::<f32>().unwrap(),
                                del_cond.unwrap().to_string(),
                                drawing_rev_no.to_string(),
                                drawing_rev_date.to_string()
                            ).post() {
                                Ok(_) => s.add_layer(Dialog::text("New Part added successfully").dismiss_button("Ok")),
                                Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                            }
                        }
                    )
                    .dismiss_button("Cancel")
            )
    }
}