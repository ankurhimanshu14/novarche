pub mod forging {

    use chrono::NaiveDate;

    use cursive::{
        Cursive,
        view::{ Nameable, Resizable },
        views::{  Dialog, EditView, ListView, SelectView, TextView },
    };

    use crate::apis::{
        engineering::part::part::Part,
        production::{
            cutting::cutting::Cutting,
            forging::forging::Forging
        }
    };

    use crate::frontend::production::cutting::cutting::display_cutting_heat;
    use crate::frontend::production::requisition::requisition::raise_requisition;

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
                                            match Forging::new(d, m.clone().unwrap().to_string(), part_code.to_string(), q).post(avail_cuttings[l][0].clone()) {
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
                                            true => match Forging::new(d, m.clone().unwrap().to_string(), part_code.to_string(), cut_qty - booked_qty as usize).post(avail_cuttings[l][0].clone()) {
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
                                    true => match Forging::new(d, m.clone().unwrap().to_string(), part_code.to_string(), tc).post(avail_cuttings[l][0].clone()) {
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
                                        true => match Forging::new(d, m.clone().unwrap().to_string(), part_code.to_string(), cut_qty - booked_qty as usize).post(avail_cuttings[l][0].clone()) {
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
}