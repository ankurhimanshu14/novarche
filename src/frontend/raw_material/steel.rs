pub mod steel {

    use cursive::{
        Cursive,
        traits::*,
        view::{ Nameable, Resizable },
        align::{ HAlign },
        views::{ Dialog, EditView, ListView, SelectView, TextView, LinearLayout },
        direction::Orientation::{ Horizontal }
    };

    use crate::apis::raw_material::{
        grades::grades::Grades,
        steel::steel::Steel
    };

    pub fn create_steels(s: &mut Cursive) {

        let g = Grades::get_list();

        s.add_layer(
            Dialog::new()
            .title("Create Steel")
            .padding_lrtb(1, 1, 1, 1)
            .content(
                ListView::new()
                .child("Item Code", EditView::new().with_name("item_code").fixed_width(30))
                .child(
                    "Grade",
                    SelectView::new()
                    .popup()
                    .h_align(HAlign::Center)
                    .autojump()
                    .with_all_str(g)
                    .on_select(|_, item| {
                        println!("{}", item);
                    })
                    .with_name("grade")
                    .fixed_width(30)
                )
                .child("Section Size", EditView::new().with_name("size").fixed_width(30))
                .child(
                    "Section Type",
                    SelectView::new()
                    .popup()
                    .h_align(HAlign::Center)
                    .autojump()
                    .with_all_str(&["DIA".to_string(), "RCS".to_string()])
                    .on_select(|_, item| {
                        println!("{}", item);
                    })
                    .with_name("section")
                    .fixed_width(30)
                )
            )
            .button(
                "Add",
                |s| {
                    let item_code = s.call_on_name("item_code", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let grade = s.call_on_name("grade", |v: &mut SelectView| {
                        v.selection()
                    }).unwrap();

                    let size = s.call_on_name("size", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let section = s.call_on_name("section", |v: &mut SelectView| {
                        v.selection()
                    }).unwrap();

                    let new_steel = Steel::new(
                        item_code.to_string(),
                        grade.unwrap().to_string(),
                        size.parse::<usize>().unwrap(),
                        section.unwrap().to_string()
                    ).post();

                    match new_steel {
                        Ok(_) =>{
                            s.pop_layer();
                            s.add_layer(Dialog::text("Steel added successfully").dismiss_button("Ok"))
                        },
                        Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                    };
                }
            )
            .dismiss_button("Cancel")
        )
    }

    pub fn get_steel_list(s: &mut Cursive) {

        let steel_list = Steel::get_steel_list();

        match steel_list.is_empty() {
            true => s.add_layer(Dialog::new().padding_lrtb(10, 10, 0, 0).content(TextView::new(format!("No Steel Created!"))).dismiss_button("Ok")),
            false => {
                s.add_layer(
                    Dialog::new()
                    .title("Steel List")
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
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Item Code")).center().fixed_width(15))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Grade Name")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Section Type")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Section Size(mm)")).center().fixed_width(10))
                                );
        
                                let mut count: usize = 0;
                                for steel in steel_list {
                                    count = count + 1;
                                    list
                                    .add_child(
                                        "",
                                        LinearLayout::new(Horizontal)
                                        .child(TextView::new(format!("{0}", count)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(format!("{0}", steel.steel_code)).fixed_width(15))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(format!("{0}", steel.grade)).fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(format!("{0}", steel.section)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(format!("{0}", steel.size)).center().fixed_width(10))
                                    )
                                }
                            }
                        )
                        .scrollable()
        
                    )
                    .dismiss_button("Ok")
                )
            }
        }
    }
}