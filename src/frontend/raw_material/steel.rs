pub mod steel {

    use cursive::{
        Cursive,
        traits::*,
        CursiveExt,
        event::Key,
        menu,
        view::{ Nameable, Resizable },
        align::{ HAlign, VAlign },
        views::{ Menubar, Dialog, EditView, ListView, SelectView, TextView, LinearLayout, Checkbox },
        direction::Orientation::{ Horizontal, Vertical }
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
                            .child(TextView::new(format!("Sr. No.")).v_align(VAlign::Center).fixed_width(10))
                            .child(TextView::new(format!("|")).v_align(VAlign::Center).fixed_width(5))
                            .child(TextView::new(format!("Item Code")).v_align(VAlign::Center).fixed_width(20))
                            .child(TextView::new(format!("|")).v_align(VAlign::Center).fixed_width(5))
                            .child(TextView::new(format!("Grade Name")).v_align(VAlign::Center).fixed_width(10))
                            .child(TextView::new(format!("|")).v_align(VAlign::Center).fixed_width(5))
                            .child(TextView::new(format!("Section Type")).v_align(VAlign::Center).fixed_width(10))
                            .child(TextView::new(format!("|")).v_align(VAlign::Center).fixed_width(5))
                            .child(TextView::new(format!("Section Size(mm)")).v_align(VAlign::Center).fixed_width(10))
                            .child(TextView::new(format!("|")).v_align(VAlign::Center).fixed_width(5))
                            .child(TextView::new(format!("Select")).v_align(VAlign::Center).fixed_width(10))
                        );

                        let mut count: usize = 0;
                        for steel in steel_list {
                            count = count + 1;
                            list
                            .add_child(
                                "",
                                LinearLayout::new(Horizontal)
                                .child(TextView::new(format!("{0}", count)).v_align(VAlign::Center).fixed_width(10))
                                .child(TextView::new(format!("|")).v_align(VAlign::Center).fixed_width(5))
                                .child(TextView::new(format!("{0}", steel.item_code)).v_align(VAlign::Center).fixed_width(20))
                                .child(TextView::new(format!("|")).v_align(VAlign::Center).fixed_width(5))
                                .child(TextView::new(format!("{0}", steel.grade)).v_align(VAlign::Center).fixed_width(10))
                                .child(TextView::new(format!("|")).v_align(VAlign::Center).fixed_width(5))
                                .child(TextView::new(format!("{0}", steel.section)).v_align(VAlign::Center).fixed_width(10))
                                .child(TextView::new(format!("|")).v_align(VAlign::Center).fixed_width(5))
                                .child(TextView::new(format!("{0}", steel.size)).v_align(VAlign::Center).fixed_width(10))
                                .child(Checkbox::new().checked().with_name("select"))
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