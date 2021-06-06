pub mod cutting {

    use chrono::NaiveDate;

    use cursive::{
        Cursive,
        traits::*,
        view::{ Nameable, Resizable },
        align::HAlign,
        views::{ Dialog, EditView, ListView, SelectView, TextView, LinearLayout, Button, FixedLayout },
        direction::Orientation::{ Horizontal, Vertical },
        Rect
    };

    use crate::apis::{
        rm_store::gate_entry::gate_entry::GateEntry,
        raw_material::steel::steel::Steel,
        engineering::part::part::Part,
        production::cutting::cutting::Cutting,
        production::requisition::requisition::Requisition,
        utils::mysql_commands::mysql_commands::check_table_exists
    };

    use crate::frontend::production::forging::forging::submit_forging_plan;
    use crate::frontend::production::requisition::requisition::raise_requisition;

    pub fn display_cutting(s: &mut Cursive) {
        s.add_layer(
            FixedLayout::new()
            .child(
                Rect::from_size((0, 0), (30,100)),
                Dialog::new().title("Sub Menu")
                .content(
                    LinearLayout::new(Vertical)
                    .with(
                        |list| {
                            list
                            .add_child(
                                LinearLayout::new(Vertical)
                                .child(Button::new_raw(
                                    format!("Pending Requisitions"),
                                    move |s| {
                                        get_request(s)
                                    }
                                ))
                                .child(Button::new_raw(
                                    format!("Cutting List"),
                                    move |s| {
                                        get_cutting_list(s)
                                    }
                                ))
                            )
                        }
                    )
                )
            )
            .child(
                Rect::from_size((30, 0), (300,100)),
                Dialog::new().title("Data Field")
            )
        )
    }

    pub fn get_request(s: &mut Cursive) {

        match check_table_exists("requisition".to_string()) {
            Ok(true) => {
                let m = Requisition::get_requisition("CUTTING".to_string());

                match m.is_empty() {
                    true => s.add_layer(Dialog::info("No requisition raised")),
                    false => {
                        s.add_layer(

                            FixedLayout::new()
                            .child(
                                Rect::from_size((0, 0), (30,100)),
                                Dialog::new().title("Sub Menu")
                                .content(
                                    LinearLayout::new(Vertical)
                                    .with(
                                        |list| {
                                            list
                                            .add_child(
                                                LinearLayout::new(Vertical)
                                                .child(Button::new_raw(
                                                    format!("Pending Requisitions"),
                                                    move |s| {
                                                        get_request(s)
                                                    }
                                                ))
                                                .child(Button::new_raw(
                                                    format!("Cutting List"),
                                                    move |s| {
                                                        get_cutting_list(s)
                                                    }
                                                ))
                                            )
                                        }
                                    )
                                )
                            )
                            .child(
                                Rect::from_size((30, 0), (300,100)),
                                Dialog::new()
                                .title("Requisition List")
                                .padding_lrtb(1, 1, 1, 0)
                                .content(
                                    ListView::new()
                                    .with(
                                        move |list| {
                                            list
                                            .add_child(
                                                "",
                                                LinearLayout::new(Horizontal)
                                                .child(TextView::new("Sr. No.").center().fixed_width(10))
                                                .child(TextView::new("|").center().fixed_width(3))
                                                .child(TextView::new("Request Raised By").center().fixed_width(20))
                                                .child(TextView::new("|").center().fixed_width(3))
                                                .child(TextView::new("Part No").center().fixed_width(10))
                                                .child(TextView::new("|").center().fixed_width(3))
                                                .child(TextView::new("Requested Quantity (Nos)").center().fixed_width(10))
                                                .child(TextView::new("|").center().fixed_width(3))
                                                .child(TextView::new("Comment").center().fixed_width(10))
                                                .child(TextView::new("|").center().fixed_width(3))
                                                .child(TextView::new("Create Cutting").center().fixed_width(20))
                                            );
                    
                                            let mut count: usize = 0;
                                            for req in m {
                                                count = count + 1;

                                                let enable_button: bool = match &*req[6].to_string() {
                                                    "Open" => false,
                                                    _ => true
                                                };

                                                list
                                                .add_child(
                                                    "",
                                                    LinearLayout::new(Horizontal)
                                                    .child(TextView::new(format!("{:?}", count)).center().fixed_width(10))
                                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                                    .child(TextView::new(req[1].to_string()).center().fixed_width(20))
                                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                                    .child(TextView::new(req[3].to_string()).center().fixed_width(10))
                                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                                    .child(TextView::new(req[4].to_string()).center().fixed_width(10))
                                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                                    .child(TextView::new(req[5].to_string()).center().fixed_width(10))
                                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                                    .child(Button::new_raw(
                                                        "   Create Plan   ",
                                                        move |s| {
                                                            let req_id = req[0].clone();

                                                            let part_no = req[3].clone();

                                                            let requested_qty = req[4].clone();

                                                            cutting_plan(s, req_id.to_string(), part_no.parse::<usize>().unwrap(), requested_qty.parse::<usize>().unwrap())
                                                        }
                                                    ).with_enabled(enable_button))
                                                )
                                            }
                                        }
                                    )
                                    .scrollable()
                    
                                )
                                .dismiss_button("Ok")
                            )
                            )
                            
                    }
                }
            },
            Ok(false) => {
                s.add_layer(Dialog::info("No pending requisitions"));
            },
            Err(e) => {
                s.add_layer(Dialog::text(format!("Error: {:?}", e)).dismiss_button("Ok"))
            }
        };
    }

    pub fn cutting_plan(s: &mut Cursive, req_id: String, p_no:usize , requested_qty: usize) {

        let h = GateEntry::get_approved_heats(p_no).unwrap();

        match h.is_empty() {
            true => {
                s.pop_layer();
                s.add_layer(Dialog::info(format!("No approved heat for part no. {} is available in the RM Store", p_no)));
            },
            false => {
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
                                .on_select(|_, item| {
                                    println!("{}", &item);
                                }
                                )
                                .with_name("machine")
                                .fixed_width(30)
                                .min_height(2)    
                            )
                            .child("Part No", TextView::new(p_no.to_string()).fixed_width(30))
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
                            .child("Planned Qty", TextView::new(requested_qty.to_string()).fixed_width(30))
                            .child("Reply", EditView::new().with_name("reply").fixed_width(30))
                    )
                    .button(
                        "Add",
                        move |s| {
                            let pd = s.call_on_name("planned_date", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
        
                            let planned_date = NaiveDate::parse_from_str(&pd.to_string(), "%d-%m-%Y").unwrap();
        
                            let machine = s.call_on_name("machine", |v: &mut SelectView| {
                                v.selection()
                            }).unwrap();
        
                            let part_code = Part::find_part_code(p_no);
        
                            let gd = s.call_on_name("grade", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
        
                            let bs = s.call_on_name("bar_size", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
        
                            let bt = s.call_on_name("section", |v: &mut SelectView| {
                                v.selection()
                            }).unwrap();

                            let rep = s.call_on_name("reply", |v: &mut EditView| {
                                v.get_content()
                            }).unwrap();
        
                            match Part::match_with_steel(
                                p_no.to_string().parse::<usize>().unwrap(),
                                gd.to_string(),
                                bs.parse::<usize>().unwrap(),
                                bt.clone().unwrap().to_string()
                            ) {
                                Ok(()) => {
                                    let steel_code = Steel::find_steel_code(gd.to_string(), bs.to_string().parse::<usize>().unwrap(), bt.unwrap().to_string());
        
                                    let heat_no = s.call_on_name("heat_no", |v: &mut SelectView| {
                                        v.selection()
                                    }).unwrap();
                
                                    match part_code.is_empty() {
                                        false => match steel_code.is_empty() {
                                            true => s.add_layer(Dialog::info("Steel is not available")),
                                            false => {
                                                let new_plan = Cutting::new(
                                                    req_id.to_string(),
                                                    planned_date,
                                                    machine.unwrap().to_string(),
                                                    part_code[0].clone(),
                                                    steel_code[0].clone(),
                                                    heat_no.clone().unwrap().to_string(),
                                                    requested_qty
                                                );
                    
                                                match Cutting::post(&new_plan, rep.to_string(), req_id.to_string()) {
                                                    Ok(0) => s.add_layer(Dialog::info("Check planning again")),
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
                                },
                                Err(e) => s.add_layer(Dialog::text(format!("{:#?}", e)).dismiss_button("Ok"))
                            }
                        }
                    )
                    .dismiss_button("Cancel")
                )
            }
        }
    }

    pub fn get_cutting_list(s: &mut Cursive) {

        let cutting_list = Cutting::get_cutting_list();

        match cutting_list.is_empty() {
            true => s.add_layer(Dialog::new().padding_lrtb(10, 10, 0, 0).content(TextView::new(format!("No planning for cutting is found"))).dismiss_button("Ok")),
            false => {
                s.add_layer(
                    FixedLayout::new()
                    .child(
                        Rect::from_size((0, 0), (30,100)),
                        Dialog::new().title("Sub Menu")
                        .content(
                            LinearLayout::new(Vertical)
                            .with(
                                |list| {
                                    list
                                    .add_child(
                                        LinearLayout::new(Vertical)
                                        .child(Button::new_raw(
                                            format!("Pending Requisitions"),
                                            move |s| {
                                                get_request(s)
                                            }
                                        ))
                                        .child(Button::new_raw(
                                            format!("Cutting List"),
                                            move |s| {
                                                get_cutting_list(s)
                                            }
                                        ))
                                    )
                                }
                            )
                        )
                    )
                    .child(
                        Rect::from_size((30, 0), (300,100)),
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
                                    .child(TextView::new(format!("Planned Date")).center().fixed_width(20))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Part No")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Heat No")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Heat Code")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Planned Qty")).center().fixed_width(15))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Actual Qty")).center().fixed_width(15))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("OK Qty")).center().fixed_width(15))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Reject Qty")).center().fixed_width(15))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new("Update").center().fixed_width(15))
                                );
        
                                let mut count: usize = 0;
                                for cut in cutting_list {
                                    count = count + 1;

                                    let enable_button: bool = match &cut[7].parse::<usize>().unwrap() {
                                        0 => true,
                                        _ => false
                                    };

                                    list
                                    .add_child(
                                        "",
                                        LinearLayout::new(Horizontal)
                                        .child(TextView::new(format!("{:?}", count)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[2]).center().fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[3]).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[4]).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[5]).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[6]).center().fixed_width(15))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[7]).center().fixed_width(15))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[8]).center().fixed_width(15))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[9]).center().fixed_width(15))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(Button::new_raw(
                                            "Update",
                                            move |s| {
                                                let r_id = &cut[0];
                                                let p_id = &cut[1];
                                                update_cutting_status(s, r_id.to_string(), p_id.to_string())
                                            }
                                        ).with_enabled(enable_button).fixed_width(15))
                                    )
                                }
                            }
                        )
                        .scrollable()
                    )
                    .dismiss_button("Ok")
                    )
                )
            }
        }
    }

    pub fn update_cutting_status(s: &mut Cursive, r: String, p: String) {

        s.add_layer(
            Dialog::new()
            .title("Update Cutting Status")
            .padding_lrtb(1, 1, 1, 1)
            .content(
                ListView::new()
                .child("Actual Qty", EditView::new().with_name("actual_qty").fixed_width(40))
                .child("Ok Qty", EditView::new().with_name("ok_qty").fixed_width(40))
                .child("End pcs wt.", EditView::new().with_name("end_pc_wt").fixed_width(40))
            )
            .button(
                "Update",
                move |s| {
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
                            match Cutting::update_cutting_status(r.clone(), p.clone(), actual_qty, ok_qty, end_pc_wt) {
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

    pub fn display_cutting_heat(s: &mut Cursive, p: usize, p_date: NaiveDate) {

        let cutting_list = Cutting::cutting_heat(p);

        match cutting_list.is_empty() {
            true => s.add_layer(Dialog::text(format!("No cutting available for part no. {}.\nRaise cutting request.", p))
            .button(
                "Create Request",
                move |s| {
                    raise_requisition(s, p, "FORGING".to_string(), "CUTTING".to_string())
                }
            ).dismiss_button("Ok")),
            false => match cutting_list.len() {
                0 => s.add_layer(Dialog::info("No cutting available")),
                _ => {
                    s.add_layer(
                        Dialog::new()
                        .title("Available Heats")
                        .padding_lrtb(1, 1, 1, 1)
                        .content(
                            ListView::new()
                            .with(
                                |list| {
                                    list.add_child(
                                        "",
                                        LinearLayout::new(Horizontal)
                                        .child(TextView::new(format!("Sr. No.")).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(format!("Part No")).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(format!("Heat No")).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(format!("Heat Code")).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(format!("Available Qty (Nos)")).center().fixed_width(20))
                                    );
            
                                    let mut count: usize = 0;
                                    for cut in cutting_list {
                                        count = count + 1;
    
                                        let enable_button: bool = match &cut[4].parse::<usize>().unwrap() {
                                            0 => false,
                                            _ => true
                                        };
            
                                        list
                                        .add_child(
                                            "",
                                            LinearLayout::new(Horizontal)
                                            .child(TextView::new(format!("{:?}", count)).center().fixed_width(10))
                                            .child(TextView::new(format!("|")).center().fixed_width(3))
                                            .child(TextView::new(&cut[1]).center().fixed_width(10))
                                            .child(TextView::new(format!("|")).center().fixed_width(3))
                                            .child(TextView::new(&cut[2]).center().fixed_width(10))
                                            .child(TextView::new(format!("|")).center().fixed_width(3))
                                            .child(TextView::new(&cut[3]).center().fixed_width(10))
                                            .child(TextView::new(format!("|")).center().fixed_width(3))
                                            .child(TextView::new(&cut[4]).center().fixed_width(20))
                                            .child(TextView::new(format!("|")).center().fixed_width(3))
                                            .child(EditView::new().with_name("planned_qty").fixed_width(20))
                                            .child(Button::new_raw(
                                                "Plan",
                                                move |s| {
                                                    let p = s.call_on_name("planned_qty", |v: &mut EditView| {
                                                        v.get_content()
                                                    }).unwrap();
    
                                                    let planned_qty = p.parse::<usize>().unwrap();
    
                                                    let part_no = cut[1].parse::<usize>().unwrap();
    
                                                    let r_id = cut[0].to_string();
    
                                                    let tot_qty = cut[4].parse::<usize>().unwrap();
    
                                                    s.pop_layer();
                                                    submit_forging_plan(r_id, s, p_date, part_no, planned_qty, tot_qty);
                                                }
                                            ).with_enabled(enable_button))
                                        )
                                    }
                                }
                            )              
                        ).dismiss_button("Cancel")
                    )
                }
            }
        }
    }
}