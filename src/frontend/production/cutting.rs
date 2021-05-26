pub mod cutting {

    use chrono::NaiveDate;

    use cursive::{
        Cursive,
        traits::*,
        CursiveExt,
        event::Key,
        menu,
        view::{ Nameable, Resizable },
        align::{ HAlign, VAlign },
        views::{ Menubar, Dialog, EditView, ListView, SelectView, TextView, TextArea, LinearLayout },
        direction::Orientation::{ Horizontal, Vertical }
    };

    use crate::apis::{
        rm_store::gate_entry::gate_entry::GateEntry,
        raw_material::steel::steel::Steel,
        engineering::part::part::Part,
        production::cutting::cutting::Cutting
    };

    pub fn plan(s: &mut Cursive) {

        let h = GateEntry::get_heat_no_list().unwrap();

        s.add_layer(
            Dialog::new()
            .title("Cutting Plan")
            .padding_lrtb(1, 1, 1, 1)
            .content(
                ListView::new()
                    .child("Date", EditView::new().with_name("planned_date").fixed_width(30))
                    .child(
                        "Machine",
                        SelectView::new()
                        .popup()
                        .autojump()
                        .with_all_str(&["BS01".to_string(), "BS02".to_string(), "BS03".to_string(), "BS04".to_string(), "BS05".to_string(), "SH-4 INCH".to_string(), "SH-5 INCH".to_string(), "SH-6 INCH".to_string()])
                        .on_select(|s, item| {
                            println!("{}", &item);
                        }
                        )
                        .with_name("machine")
                        .fixed_width(30)
                        .min_height(2)    
                    )
                    .child("Part No", EditView::new().with_name("part_no").fixed_width(30))
                    .child("Steel Grade", EditView::new().with_name("grade").fixed_width(30))
                    .child("Bar Size", EditView::new().with_name("bar_size").fixed_width(30))
                    .child(
                        "Bar Type",
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
                    .child(
                        "Heat No",
                        SelectView::new()
                        .popup()
                        .h_align(HAlign::Center)
                        .autojump()
                        .with_all_str(h)
                        .on_select(|_, item| {
                            println!("{}", item);
                        })
                        .with_name("heat_no")
                        .fixed_width(30)
                    )
                    .child("Planned Qty", EditView::new().with_name("planned_qty").fixed_width(30))
                    .child("Actual Qty", EditView::new().with_name("actual_qty").fixed_width(30))
                    .child("OK Qty", EditView::new().with_name("ok_qty").fixed_width(30))
                    .child("End pcs Wt", EditView::new().with_name("end_pc_wt").fixed_width(30))
            )
            .button(
                "Add",
                |s| {
                    let pd = s.call_on_name("planned_date", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let planned_date = NaiveDate::parse_from_str(&pd.to_string(), "%d-%m-%Y").unwrap();

                    let machine = s.call_on_name("machine", |v: &mut SelectView| {
                        v.selection()
                    }).unwrap();

                    let p_no = s.call_on_name("part_no", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let part_code = Part::find_part_code(p_no.to_string().parse::<usize>().unwrap());

                    let gd = s.call_on_name("grade", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let bs = s.call_on_name("bar_size", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let bt = s.call_on_name("section", |v: &mut SelectView| {
                        v.selection()
                    }).unwrap();

                    let steel_code = Steel::find_steel_code(gd.to_string(), bs.to_string().parse::<usize>().unwrap(), bt.unwrap().to_string());

                    let heat_no = s.call_on_name("heat_no", |v: &mut SelectView| {
                        v.selection()
                    }).unwrap();

                    let planned_qty = s.call_on_name("planned_qty", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let actual_qty = s.call_on_name("actual_qty", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let ok_qty = s.call_on_name("ok_qty", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let end_pc_wt = s.call_on_name("end_pc_wt", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    match part_code.is_empty() || steel_code.is_empty() {
                        true => s.add_layer(Dialog::info("Part or Steel list is not available")),
                        false => {
                            let new_plan = Cutting::new(
                                planned_date,
                                machine.unwrap().to_string(),
                                part_code[0].clone(),
                                steel_code[0].clone(),
                                heat_no.unwrap().to_string(),
                                planned_qty.to_string().parse::<usize>().unwrap(),
                                Some(actual_qty.to_string().parse::<usize>().unwrap()),
                                ok_qty.to_string().parse::<usize>().unwrap(),
                                Some(end_pc_wt.to_string().parse::<f64>().unwrap()),
                            );

                            match Cutting::post(&new_plan) {
                                Ok(m) =>{
                                    s.pop_layer();
                                    s.add_layer(Dialog::text(format!("Plan added successfully. Insert ID: {}", m)).dismiss_button("Ok"))
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