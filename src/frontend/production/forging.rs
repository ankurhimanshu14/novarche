pub mod forging {

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
        engineering::part::part::Part,
        production::cutting::cutting::Cutting,
        production::forging::forging::Forging
    };

    use crate::frontend::raw_material::steel::steel::create_steels;

    use crate::apis::utility_tools::parse::parse::parse_from_row;

    pub fn get_input_material(s: &mut Cursive) {
        let cutting_inv = Cutting::ready_to_forge();

        match cutting_inv.is_empty() {
            true => s.add_layer(Dialog::new().padding_lrtb(10, 10, 0, 0).content(TextView::new(format!("No Cutting Inventory!"))).dismiss_button("Ok")),
            false => {
                s.add_layer(
                    Dialog::new()
                    .title("Cutting Inventory List")
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
                                    .child(TextView::new(format!("Part No")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Heat No")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("OK Qty (Nos)")).center().fixed_width(20))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new("Select").center().fixed_width(20))
                                );
        
                                let mut count: usize = 0;
                                for cut in cutting_inv {
                                    count = count + 1;

                                    let enable: bool = match cut[5].parse::<u32>().unwrap() {
                                        0 => true,
                                        _ => false
                                    };

                                    list
                                    .add_child(
                                        "",
                                        LinearLayout::new(Horizontal)
                                        .child(TextView::new(format!("{:?}", count)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[2]).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[3]).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[4]).center().fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[5]).center().fixed_width(20))
                                        .child(Button::new_raw(
                                            "Add to production",
                                            move |s| {
                                                let r_id = &cut[0];
                                                let c_id = &cut[1];
                                                forging_plan(s, r_id.to_string(), c_id.to_string())
                                            }
                                        ).with_enabled(enable))
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

    pub fn forging_plan(s: &mut Cursive, r: String, c: String) {

        s.add_layer(
            Dialog::new()
            .title("Forging Plan")
            .padding_lrtb(1, 1, 1, 1)
            .content(
                ListView::new()
                    .child("Date", EditView::new().with_name("planned_date").fixed_width(30))
                    .child(
                        "Machine",
                        SelectView::new()
                        .popup()
                        .autojump()
                        .with_all_str(&["1.25 T Hammer".to_string(), "1.50 T Hammer".to_string(), "2.0 T Hammer (O)".to_string(), "2.0 T Hammer (N)".to_string(), "2.5 T Hammer".to_string(), "1600 T Press (O)".to_string(), "1600 T Press (N)".to_string(), "2500 T Press".to_string()])
                        .on_select(|s, item| {
                            println!("{}", &item);
                        }
                        )
                        .with_name("machine")
                        .fixed_width(30)
                        .min_height(2)    
                    )
                    .child("Part No", EditView::new().with_name("part_no").fixed_width(30))
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

                    let planned_qty = s.call_on_name("planned_qty", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    // let cut_wt = Part::get_cut_wt(p_no.to_string().parse::<usize>().unwrap());
                    // let avail_qty = GateEntry::get_avail_qty(heat_no.clone().unwrap().to_string());
                    // let planned_wt = planned_qty.to_string().parse::<f64>().unwrap() * cut_wt.clone();

                    // let est_prod: usize = (avail_qty / cut_wt) as usize;

                    match part_code.is_empty() {
                        true => s.add_layer(Dialog::info("Part not defined")),
                        false => {
                            let new_plan = Forging::new(
                                planned_date,
                                machine.unwrap().to_string(),
                                part_code[0].clone(),
                                planned_qty.to_string().parse::<usize>().unwrap()
                            );

                            match Forging::post(&new_plan) {
                                Ok(0) => s.add_layer(Dialog::info("Check planning again")),
                                Ok(m) =>{
                                    s.pop_layer();
                                    s.add_layer(Dialog::text(format!("Plan added successfully. Insert ID: {}", m)).dismiss_button("Ok"));
                                    get_input_material(s);
                                },
                                Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                            }
                        }
                    }
                }
            )
            .dismiss_button("Cancel")
        )
    }

    // pub fn get_forging_list(s: &mut Cursive) {

    //     let cutting_list = Cutting::get_cutting_list();

    //     match cutting_list.is_empty() {
    //         true => s.add_layer(Dialog::new().padding_lrtb(10, 10, 0, 0).content(TextView::new(format!("No Cutting Plan Created!"))).dismiss_button("Ok")),
    //         false => {
    //             s.add_layer(
    //                 Dialog::new()
    //                 .title("Cutting List")
    //                 .padding_lrtb(1, 1, 1, 0)
    //                 .content(
    //                     ListView::new()
    //                     .with(
    //                         |list| {
    //                             list
    //                             .add_child(
    //                                 "",
    //                                 LinearLayout::new(Horizontal)
    //                                 .child(TextView::new(format!("Sr. No.")).center().fixed_width(10))
    //                                 .child(TextView::new(format!("|")).center().fixed_width(3))
    //                                 .child(TextView::new(format!("Planned Date")).center().fixed_width(20))
    //                                 .child(TextView::new(format!("|")).center().fixed_width(3))
    //                                 .child(TextView::new(format!("Part No")).center().fixed_width(10))
    //                                 .child(TextView::new(format!("|")).center().fixed_width(3))
    //                                 .child(TextView::new(format!("Heat No")).center().fixed_width(10))
    //                                 .child(TextView::new(format!("|")).center().fixed_width(3))
    //                                 .child(TextView::new(format!("Planned Qty (Nos)")).center().fixed_width(20))
    //                                 .child(TextView::new(format!("|")).center().fixed_width(3))
    //                                 .child(TextView::new(format!("Actual Qty (Nos)")).center().fixed_width(20))
    //                                 .child(TextView::new(format!("|")).center().fixed_width(3))
    //                                 .child(TextView::new(format!("OK Qty (Nos)")).center().fixed_width(20))
    //                                 .child(TextView::new(format!("|")).center().fixed_width(3))
    //                                 .child(TextView::new(format!("Reject Qty (Nos)")).center().fixed_width(20))
    //                                 .child(TextView::new("Update Production").center().fixed_width(20))
    //                             );
        
    //                             let mut count: usize = 0;
    //                             for cut in cutting_list {
    //                                 count = count + 1;

    //                                 let enable_button: bool = match &cut[6].parse::<usize>().unwrap() {
    //                                     0 => true,
    //                                     _ => false
    //                                 };

    //                                 list
    //                                 .add_child(
    //                                     "",
    //                                     LinearLayout::new(Horizontal)
    //                                     .child(TextView::new(format!("{:?}", count)).center().fixed_width(10))
    //                                     .child(TextView::new(format!("|")).center().fixed_width(3))
    //                                     .child(TextView::new(&cut[2]).center().fixed_width(20))
    //                                     .child(TextView::new(format!("|")).center().fixed_width(3))
    //                                     .child(TextView::new(&cut[3]).center().fixed_width(10))
    //                                     .child(TextView::new(format!("|")).center().fixed_width(3))
    //                                     .child(TextView::new(&cut[4]).center().fixed_width(10))
    //                                     .child(TextView::new(format!("|")).center().fixed_width(3))
    //                                     .child(TextView::new(&cut[5]).center().fixed_width(20))
    //                                     .child(TextView::new(format!("|")).center().fixed_width(3))
    //                                     .child(TextView::new(&cut[6]).center().fixed_width(20))
    //                                     .child(TextView::new(format!("|")).center().fixed_width(3))
    //                                     .child(TextView::new(&cut[7]).center().fixed_width(20))
    //                                     .child(TextView::new(format!("|")).center().fixed_width(3))
    //                                     .child(TextView::new(&cut[8]).center().fixed_width(20))
    //                                     .child(Button::new_raw(
    //                                         "        Update       ",
    //                                         move |s| {
    //                                             let r_id = &cut[0];
    //                                             let p_id = &cut[1];
    //                                             update_cutting_status(s, r_id.to_string(), p_id.to_string())
    //                                         }
    //                                     ).with_enabled(enable_button))
    //                                 )
    //                             }
    //                         }
    //                     )
    //                     .scrollable()
        
    //                 )
    //                 .dismiss_button("Ok")
    //             )
    //         }
    //     }
    // }

    // pub fn update_cutting_status(s: &mut Cursive, r: String, p: String) {

    //     s.add_layer(
    //         Dialog::new()
    //         .title("Update Cutting Status")
    //         .padding_lrtb(1, 1, 1, 1)
    //         .content(
    //             ListView::new()
    //             .child("Actual Qty", EditView::new().with_name("actual_qty").fixed_width(40))
    //             .child("Ok Qty", EditView::new().with_name("ok_qty").fixed_width(40))
    //             .child("End pcs wt.", EditView::new().with_name("end_pc_wt").fixed_width(40))
    //         )
    //         .button(
    //             "Update",
    //             move |s| {
    //                 let actual_qty = s.call_on_name("actual_qty", |v: &mut EditView|{
    //                     v.get_content()
    //                 }).unwrap();

    //                 let actual_qty = actual_qty.parse::<usize>().unwrap();

    //                 let ok_qty = s.call_on_name("ok_qty", |v: &mut EditView|{
    //                     v.get_content()
    //                 }).unwrap();

    //                 let ok_qty = ok_qty.parse::<usize>().unwrap();

    //                 let end_pc_wt = s.call_on_name("end_pc_wt", |v: &mut EditView|{
    //                     v.get_content()
    //                 }).unwrap();

    //                 let end_pc_wt = end_pc_wt.parse::<f64>().unwrap();

    //                 match ok_qty <= actual_qty {
    //                     true => {
    //                         match Cutting::update_cutting_status(r.clone(), p.clone(), actual_qty, ok_qty, end_pc_wt) {
    //                             Ok(_) => {
    //                                 s.pop_layer();
    //                                 s.add_layer(Dialog::text("Cutting Status updated").button("Ok", |c| {
    //                                     c.pop_layer();
    //                                     c.pop_layer();
    //                                     get_cutting_list(c);
    //                                 }))
    //                             },
    //                             Err(e) => s.add_layer(Dialog::info(format!("Error: {}", e)))
    //                         }
    //                     },
    //                     false => s.add_layer(Dialog::info("OK Qty is more than Actual Production"))
    //                 }


    //             }
    //         )
    //         .dismiss_button("Cancel")
    //     )
    // }
}