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
        views::{ Menubar, Dialog, EditView, ListView, SelectView, TextView, TextArea, LinearLayout, Button },
        direction::Orientation::{ Horizontal, Vertical }
    };

    use crate::apis::{
        rm_store::gate_entry::gate_entry::GateEntry,
        raw_material::steel::steel::Steel,
        engineering::part::part::Part,
        production::cutting::cutting::Cutting
    };

    use crate::frontend::raw_material::steel::steel::create_steels;

    use crate::apis::utility_tools::parse::parse::parse_from_row;

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

                    let cut_wt = Part::get_cut_wt(p_no.to_string().parse::<usize>().unwrap());
                    let avail_qty = GateEntry::get_avail_qty(heat_no.clone().unwrap().to_string());
                    let planned_wt = planned_qty.to_string().parse::<f64>().unwrap() * cut_wt.clone();

                    let est_prod: usize = (avail_qty / cut_wt) as usize;

                    match part_code.is_empty() {
                        false => match steel_code.is_empty() {
                            true => s.add_layer(Dialog::info("Steel is not available")),
                            false => {
                                let new_plan = Cutting::new(
                                    planned_date,
                                    machine.unwrap().to_string(),
                                    part_code[0].clone(),
                                    steel_code[0].clone(),
                                    heat_no.clone().unwrap().to_string(),
                                    planned_qty.to_string().parse::<usize>().unwrap()
                                );
    
                                match Cutting::post(&new_plan, planned_wt) {
                                    Ok(0) => {
                                        s.pop_layer();
                                        s.add_layer(Dialog::text(format!("Inventory Short. Max production qty: {}", est_prod)).dismiss_button("Ok"));
                                    },
                                    Ok(m) =>{
                                        s.pop_layer();
                                        s.add_layer(Dialog::text(format!("Plan added successfully. Insert ID: {}", m)).dismiss_button("Ok"))
                                    },
                                    Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                                }
                            }
                        },
                        true => s.add_layer(Dialog::info("Part is not available"))
                    }
                }
            )
            .dismiss_button("Cancel")
        )
    }

    pub fn get_cutting_list(s: &mut Cursive) {

        let cutting_list = Cutting::get_cutting_list();

        match cutting_list.is_empty() {
            true => s.add_layer(Dialog::new().padding_lrtb(10, 10, 0, 0).content(TextView::new(format!("No Cutting Plan Created!"))).dismiss_button("Ok")),
            false => {
                s.add_layer(
                    Dialog::new()
                    .title("Cutting List")
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
                                    .child(TextView::new(format!("Cutting ID")).center().fixed_width(20))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Planned Date")).center().fixed_width(20))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Part No")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Heat No")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Planned Qty (Nos)")).center().fixed_width(20))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Actual Qty (Nos)")).center().fixed_width(20))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("OK Qty (Nos)")).center().fixed_width(20))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Reject Qty (Nos)")).center().fixed_width(20))
                                    .child(TextView::new("Update Production").center().fixed_width(20))
                                );
        
                                let mut count: usize = 0;
                                for cut in &cutting_list {
                                    count = count + 1;

                                    let enable_button: bool = match &cut[5].parse::<usize>().unwrap() {
                                        0 => true,
                                        _ => false
                                    };

                                    list
                                    .add_child(
                                        "",
                                        LinearLayout::new(Horizontal)
                                        .child(TextView::new(format!("{:?}", count)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[0]).center().fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[1]).center().fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[2]).fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[3]).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[4]).center().fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[5]).center().fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[6]).center().fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[7]).center().fixed_width(20))
                                        .child(Button::new_raw("        Update       ", |s| { update_cutting_status(s)}).with_enabled(enable_button))
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

    pub fn update_cutting_status(s: &mut Cursive) {

        s.add_layer(
            Dialog::new()
            .title("Update Cutting Status")
            .padding_lrtb(1, 1, 1, 1)
            .content(
                ListView::new()
                .child("Cutting ID", EditView::new().with_name("cutting_id").fixed_width(30))
                .child("Actual Qty", EditView::new().with_name("actual_qty").fixed_width(30))
                .child("Ok Qty", EditView::new().with_name("ok_qty").fixed_width(30))
                .child("End pcs wt.", EditView::new().with_name("end_pc_wt").fixed_width(30))
            )
            .button(
                "Update",
                |s| {
                    let cutting_id = s.call_on_name("cutting_id", |v: &mut EditView|{
                        v.get_content()
                    }).unwrap();

                    let cutting_id = cutting_id.parse::<usize>().unwrap();

                    let actual_qty = s.call_on_name("actual_qty", |v: &mut EditView|{
                        v.get_content()
                    }).unwrap();

                    let actual_qty = actual_qty.parse::<usize>().unwrap();

                    let ok_qty = s.call_on_name("ok_qty", |v: &mut EditView|{
                        v.get_content()
                    }).unwrap();

                    let ok_qty = ok_qty.parse::<usize>().unwrap();

                    let end_pc_wt = s.call_on_name("end_pc_wt", |v: &mut EditView|{
                        v.get_content()
                    }).unwrap();

                    let end_pc_wt = end_pc_wt.parse::<f64>().unwrap();

                    match ok_qty <= actual_qty {
                        true => {
                            match Cutting::update_cutting_status(cutting_id, actual_qty, ok_qty, end_pc_wt) {
                                Ok(_) => {
                                    s.pop_layer();
                                    s.add_layer(Dialog::text("Cutting Status updated").button("Ok", |c| {
                                        c.pop_layer();
                                        c.pop_layer();
                                        get_cutting_list(c);
                                    }))
                                },
                                Err(e) => s.add_layer(Dialog::info(format!("Error: {}", e)))
                            }
                        },
                        false => s.add_layer(Dialog::info("OK Qty is more than Actual Production"))
                    }


                }
            )
            .dismiss_button("Cancel")
        )
    }
}