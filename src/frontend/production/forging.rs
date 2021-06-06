pub mod forging {

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

    use crate::apis::utils::gen_uuid::gen_uuid::generate_uuid;

    use crate::apis::{
        engineering::part::part::Part,
        production::{
            cutting::cutting::Cutting,
            forging::forging::Forging
        }
    };

    use crate::frontend::production::cutting::cutting::display_cutting_heat;
    use crate::frontend::production::requisition::requisition::raise_requisition;

    pub fn display_forging(s: &mut Cursive) {
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
                                    format!("Create Plan"),
                                    move |s| {
                                        forging_plan(s)
                                    }
                                ))
                                .child(Button::new_raw(
                                    format!("Update Plan"),
                                    move |s| {
                                        get_forging_list(s)
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

    pub fn forging_plan(s: &mut Cursive) {
        s.add_layer(
            Dialog::new()
            .title("Create Forging Plan")
            .padding_lrtb(1, 1, 1, 1)
            .content(
                ListView::new()
                .child("Planning Date", EditView::new().with_name("planned_date").fixed_width(30))
                .child("Part No", EditView::new().with_name("part_no").fixed_width(30))
            )
            .button(
                "Select Cutting",
                move |s| {
                    let planned_date = s.call_on_name("planned_date", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let planned_date = NaiveDate::parse_from_str(&planned_date.to_string(), "%d-%m-%Y").unwrap();

                    let p_no = s.call_on_name("part_no", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let part_no = p_no.parse::<usize>().unwrap();

                    let part_code = Part::find_part_code(part_no);

                    match part_code.len() {
                        0 => s.add_layer(Dialog::info("Part not defined")),
                        1 => {
                            s.pop_layer();
                            display_cutting_heat(s, part_no , planned_date);
                        },
                        _ => s.add_layer(Dialog::info("Error in fetching part list"))
                    }
                }
            )
            .dismiss_button("Ok")
        )
    }

    pub fn submit_forging_plan(r: String, s: &mut Cursive, d: NaiveDate, p: usize, q: usize, t: usize) {
        s.add_layer(
            Dialog::new()
            .title("Forging Plan")
            .padding_lrtb(1, 1, 1, 1)
            .content(
                ListView::new()
                .child("Date", TextView::new(d.to_string()).fixed_width(30))
                .child(
                    "Machine",
                    SelectView::new()
                    .popup()
                    .autojump()
                    .with_all_str(&[
                        "1.25 T Hammer".to_string(),
                        "1.50 T Hammer".to_string(),
                        "2.00 T Hammer(O)".to_string(),
                        "2.00 T Hammer(N)".to_string(),
                        "2.50 T Hammer".to_string(),
                        "1600 T Press(O)".to_string(),
                        "1600 T Press(N)".to_string(),
                        "2500 T Press".to_string()
                    ])
                    .on_select(|_, item| {
                        println!("{}", item)
                    })
                    .with_name("machine")
                    .fixed_width(30)
                )
                .child("Part No", TextView::new(p.to_string()).fixed_width(30))
                .child("Planned Qty (Nos)", TextView::new(q.to_string()).fixed_width(30))
            )
            .button(
                "Submit",
                move |s| {

                    let f_id = generate_uuid();

                    let m = s.call_on_name("machine", |v: &mut SelectView| {
                        v.selection()
                    }).unwrap();

                    let part_code = &Part::find_part_code(p.clone())[0];

                    //Return the vector of vectors of cutting id, part no and avail qty as strings
                    let avail_cuttings = Cutting::avail_qty_list(r.to_string(), p);

                    let mut q = q;

                    for l in 0..avail_cuttings.len() {

                        let cut_qty = avail_cuttings[l][2].parse::<usize>().unwrap().clone();

                        let in_plan = Forging::qty_in_plan(p.clone());

                        let c_id = avail_cuttings[l][0].clone();

                        let part_no = avail_cuttings[l][1].parse::<usize>().unwrap().clone();

                        while q > 0 {

                            let tc = t - in_plan as usize;

                            let booked_qty = match Forging::booked_qty(c_id.clone().to_string(), p) {
                                -1 => 0,
                                v => v
                            };

                            match tc > q {
                                true => {                                    
                                    match cut_qty - booked_qty as usize > q {
                                        true => {
                                            match Forging::new(f_id.clone(), d, m.clone().unwrap().to_string(), part_code.to_string(), q).post(avail_cuttings[l][0].clone()) {
                                                Ok(i) => {
                                                    s.pop_layer();
                                                    s.add_layer(Dialog::text(format!("Forging Plan created for {} nos.\nInsert ID: {}.", q, i)).dismiss_button("Ok"));
                                                    q = 0;
                                                },
                                                Err(e) => {
                                                    s.pop_layer();
                                                    s.add_layer(Dialog::text(format!("Error : {}", e)).dismiss_button("Ok"));
                                                }
                                            }
                                        },
                                        false => match cut_qty > booked_qty as usize {
                                            true => match Forging::new(f_id.clone(), d, m.clone().unwrap().to_string(), part_code.to_string(), cut_qty - booked_qty as usize).post(avail_cuttings[l][0].clone()) {
                                                Ok(i) => {
                                                    s.pop_layer();
                                                    s.add_layer(Dialog::text(format!("Forging Plan created for {} nos.\nInsert ID: {}.", cut_qty - booked_qty as usize, i)).dismiss_button("Ok"));
                                                    q = q + booked_qty as usize - cut_qty;
                                                },
                                                Err(e) => {
                                                    s.pop_layer();
                                                    s.add_layer(Dialog::text(format!("Error : {}", e)).dismiss_button("Ok"));
                                                }
                                            },
                                            false => {
                                                s.pop_layer();
                                                s.add_layer(Dialog::text(format!("All cuttings are booked for forging.\nRaise cutting request for {} nos.", q))
                                                .button(
                                                    "Create Request",
                                                    move |s| {
                                                        raise_requisition(s, part_no, "FORGING".to_string(), "CUTTING".to_string())
                                                    }
                                                )
                                                .dismiss_button("Cancel")
                                            );
                                                break;
                                            }
                                        }
                                    }
                                },
                                false => match cut_qty - booked_qty as usize > q {
                                    true => match Forging::new(f_id.clone(), d, m.clone().unwrap().to_string(), part_code.to_string(), tc).post(avail_cuttings[l][0].clone()) {
                                        Ok(i) => {
                                            s.pop_layer();
                                            s.add_layer(Dialog::text(format!("Forging Plan created for {} nos.\nInsert ID: {}.\nRaise cutting request for minimum {} nos.", tc, i, q - tc))
                                            .button(
                                                "Create Request",
                                                move |s| {
                                                    raise_requisition(s, part_no, "FORGING".to_string(), "CUTTING".to_string())
                                                }
                                            )
                                            .dismiss_button("Cancel")
                                        );
                                            q = q -tc;
                                        },
                                        Err(e) => {
                                            s.pop_layer();
                                            s.add_layer(Dialog::text(format!("Error : {}", e)).dismiss_button("Ok"));
                                        }
                                    },
                                    false => match cut_qty > booked_qty as usize {
                                        true => match Forging::new(f_id.clone(), d, m.clone().unwrap().to_string(), part_code.to_string(), cut_qty - booked_qty as usize).post(avail_cuttings[l][0].clone()) {
                                            Ok(i) => {
                                                s.pop_layer();
                                                s.add_layer(Dialog::text(format!("Forging Plan created for {} nos.\nInsert ID: {}.", cut_qty - booked_qty as usize, i)).dismiss_button("Ok"));
                                                q = q + booked_qty as usize - cut_qty;
                                            },
                                            Err(e) => {
                                                s.pop_layer();
                                                s.add_layer(Dialog::text(format!("Error : {}", e)).dismiss_button("Ok"));
                                            }
                                        },
                                        false => {
                                            s.pop_layer();
                                            s.add_layer(Dialog::text(format!("All cuttings are booked for forging.\nRaise cutting request for {} nos.", q))
                                            .button(
                                                "Create Request",
                                                move |s| {
                                                    raise_requisition(s, part_no, "FORGING".to_string(), "CUTTING".to_string())
                                                }
                                            )
                                            .dismiss_button("Cancel")
                                        );
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            )
            .dismiss_button("Cancel")
        )
    }

    pub fn get_forging_list(s: &mut Cursive) {

        let forging_list = Forging::get_forging_list();

        match forging_list.is_empty() {
            true => s.add_layer(Dialog::new().padding_lrtb(10, 10, 0, 0).content(TextView::new(format!("No planning for forging is found"))).dismiss_button("Ok")),
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
                                            format!("Cutting List"),
                                            move |s| {
                                                get_forging_list(s)
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
                    .title("Forging List")
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
                                for forg in forging_list {
                                    count = count + 1;

                                    let enable_button: bool = match &forg[5].parse::<usize>().unwrap() {
                                        0 => true,
                                        _ => false
                                    };

                                    list
                                    .add_child(
                                        "",
                                        LinearLayout::new(Horizontal)
                                        .child(TextView::new(format!("{:?}", count)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&forg[2]).center().fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&forg[3]).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&forg[4]).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&forg[5]).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&forg[6]).center().fixed_width(15))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&forg[7]).center().fixed_width(15))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(Button::new_raw(
                                            "Update",
                                            move |s| {
                                                let c_id = &forg[0];
                                                let f_id = &forg[1];
                                                update_forging_status(s, c_id.to_string(), f_id.to_string())
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

    pub fn update_forging_status(s: &mut Cursive, c: String, f: String) {

        s.add_layer(
            Dialog::new()
            .title("Update Forging Status")
            .padding_lrtb(1, 1, 1, 1)
            .content(
                ListView::new()
                .child("Actual Qty", EditView::new().with_name("actual_qty").fixed_width(40))
                .child("Ok Qty", EditView::new().with_name("ok_qty").fixed_width(40))
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

                    match ok_qty <= actual_qty {
                        true => {
                            match Forging::update_forging_status(c.clone(), f.clone(), actual_qty, ok_qty) {
                                Ok(_) => {
                                    s.pop_layer();
                                    s.add_layer(Dialog::text("Forging Status updated").button("Ok", |c| {
                                        c.pop_layer();
                                        c.pop_layer();
                                        get_forging_list(c);
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