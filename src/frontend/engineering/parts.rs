pub mod parts {

    use cursive::{
        Cursive,
        traits::*,
        view::{ Nameable, Resizable },
        align::{ VAlign },
        views::{ Dialog, EditView, ListView, SelectView, TextView, LinearLayout },
        direction::Orientation::{ Horizontal }
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
                            .child("Item Code", EditView::new().with_name("part_code").fixed_width(30))
                            .child("Part No", EditView::new().with_name("part_no").fixed_width(30))
                            .child("Part Name", EditView::new().with_name("part_name").fixed_width(30))
                            .child(
                                "Material Grade",
                                SelectView::new()
                                    .popup()
                                    .v_align(VAlign::Center)
                                    .autojump()
                                    .with_all_str(grd)
                                    .on_select(|_, item| {
                                        println!("{}", &item)
                                    })
                                    .with_name("grade")
                                    .fixed_width(30)
                                )
                            .child("Section Size", EditView::new().with_name("sec_size").fixed_width(30))
                            .child(
                                "Section Type",
                                SelectView::new()
                                    .popup()
                                    .v_align(VAlign::Center)
                                    .autojump()
                                    .with_all_str(&["DIA".to_string(), "RCS".to_string()])
                                    .on_select(|_, item| {
                                        println!("{}", &item)
                                    })
                                    .with_name("sec_type")
                                    .fixed_width(30)
                                )
                            .child("Forging Weight", EditView::new().with_name("forging_wt").fixed_width(30))
                            .child("Cut Weight", EditView::new().with_name("cut_wt").fixed_width(30))
                            .child(
                                "Delivery Condition",
                                SelectView::new()
                                .popup()
                                .v_align(VAlign::Center)
                                .autojump()
                                .with_all_str(&["Normalized".to_string(), "Isothermal Annealed".to_string(), "Annealed".to_string(), "As Forged".to_string()])
                                .on_select(|_, item| {
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
                            let part_code = s.call_on_name("part_code", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();

                            let part_no = s.call_on_name("part_no", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
                            let part_name = s.call_on_name("part_name", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
                            let grade = s.call_on_name("grade", |v: &mut SelectView| {
                                v.selection()
                            }).unwrap();
                            let sec_size = s.call_on_name("sec_size", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
                            let sec_type = s.call_on_name("sec_type", |v: &mut SelectView| {
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
                                part_code.to_string(),
                                part_no.parse::<usize>().unwrap(),
                                part_name.to_string(),
                                grade.unwrap().to_string(),
                                sec_size.parse::<usize>().unwrap(),
                                sec_type.unwrap().to_string(),
                                forging_wt.parse::<f32>().unwrap(),
                                cut_wt.parse::<f32>().unwrap(),
                                del_cond.unwrap().to_string(),
                                drawing_rev_no.to_string(),
                                drawing_rev_date.to_string()
                            ).post() {
                                Ok(_) => { s.pop_layer();
                                s.add_layer(Dialog::text("New Part added successfully").button(
                                    "Ok",
                                    |c| {
                                        c.pop_layer();
                                        create_parts(c);
                                    }
                                ))},
                                Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                            }
                        }
                    )
                    .dismiss_button("Cancel")
            )
    }

    pub fn get_part_list(s: &mut Cursive) {

        let part_list = Part::get_part_list();

        match part_list.is_empty() {
            true => s.add_layer(Dialog::new().padding_lrtb(10, 10, 0, 0).content(TextView::new(format!("No Part Created!"))).dismiss_button("Ok")),
            false => {
                s.add_layer(
                    Dialog::new()
                    .title("Part List")
                    .padding_lrtb(1, 1, 1, 0)
                    .content(
                        ListView::new()
                        .with(
                            |list| {
                                list
                                .add_child(
                                    "",
                                    LinearLayout::new(Horizontal)
                                    .child(TextView::new(format!("Sr. No.")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Item Code")).center().fixed_width(15))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Part No")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Part Name")).center().fixed_width(20))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Grade")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Bar Size")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Section")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Cut Weight")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Forging Weight")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Delivery Condition")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Dwg. Rev. No")).center().fixed_width(15))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Dwg. Rev. Date")).center().fixed_width(15))
                                );
        
                                let mut count: usize = 0;
                                for part in part_list {
                                    count = count + 1;
                                    list
                                    .add_child(
                                        "",
                                        LinearLayout::new(Horizontal)
                                        .child(TextView::new(format!("{0}", count)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", part.part_code)).fixed_width(15))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", part.part_no)).fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", part.part_name)).fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", part.grade)).fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", part.sec_size)).fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", part.sec_type)).fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", part.cut_wt)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", part.forging_wt)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", part.del_cond)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", part.drawing_rev_no)).center().fixed_width(15))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", part.drawing_rev_date)).center().fixed_width(15))
                                    )
                                }
                            }
                        )
                        .min_height(10)
                        .scrollable()
        
                    )
                    .dismiss_button("Ok")
                )
            }
        }
    }
}