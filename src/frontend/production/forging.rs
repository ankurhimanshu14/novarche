pub mod forging {

    use chrono::NaiveDate;

    use cursive::{
        Cursive,
        traits::*,
        view::{ Nameable, Resizable },
        views::{  Dialog, EditView, ListView, SelectView, TextView, LinearLayout, Button },
        direction::Orientation::{ Horizontal }
    };

    use crate::apis::{
        engineering::part::part::Part,
        production::cutting::cutting::Cutting,
        production::forging::forging::Forging
    };

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
                                    .child(TextView::new(format!("Heat Code")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("OK Qty (Nos)")).center().fixed_width(20))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new("Select").center().fixed_width(20))
                                );
        
                                let mut count: usize = 0;
                                for cut in cutting_inv {
                                    count = count + 1;

                                    let enable: bool = match cut[6].parse::<u32>().unwrap() {
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
                                        .child(TextView::new(&cut[4]).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&cut[5]).center().fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(Button::new_raw(
                                            "Add to production",
                                            move |s| {
                                                let p_no = &cut[2].parse::<usize>().unwrap();
                                                let q = &cut[5].parse::<usize>().unwrap();
                                                forging_plan(s, *p_no, *q)
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

    pub fn forging_plan(s: &mut Cursive, p_no: usize, qty: usize) {

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
                        .with_all_str(
                            &[
                                "1.25 T Hammer".to_string(),
                                "1.50 T Hammer".to_string(),
                                "2.0 T Hammer (O)".to_string(),
                                "2.0 T Hammer (N)".to_string(),
                                "2.5 T Hammer".to_string(),
                                "1600 T Press (O)".to_string(),
                                "1600 T Press (N)".to_string(),
                                "2500 T Press".to_string()
                            ]
                        )
                        .on_select(|_, item| {
                            println!("{}", &item);
                        }
                        )
                        .with_name("machine")
                        .fixed_width(30)
                        .min_height(2)    
                    )
                    .child("Part No", TextView::new(p_no.to_string()).center().fixed_width(30))
                    .child("Available Qty", TextView::new(qty.to_string()).center().fixed_width(30))
                    .child("Planned Qty", EditView::new().with_name("planned_qty").fixed_width(30))
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

                    let part_code = Part::find_part_code(p_no.to_string().parse::<usize>().unwrap());

                    let planned_qty = s.call_on_name("planned_qty", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let planned_qty = planned_qty.parse::<usize>().unwrap();

                    match part_code.is_empty() {
                        true => s.add_layer(Dialog::info("Part not defined")),
                        false => {
                            match planned_qty <= qty {
                                true => {
                                    let new_plan = Forging::new(
                                        planned_date,
                                        machine.unwrap().to_string(),
                                        part_code[0].clone(),
                                        planned_qty
                                    );
        
                                    match Forging::post(&new_plan) {
                                        Ok(0) => s.add_layer(Dialog::info("Check planning again")),
                                        Ok(m) =>{
                                            s.pop_layer();
                                            s.pop_layer();
                                            s.pop_layer();
                                            s.add_layer(Dialog::text(format!("Plan added successfully. Insert ID: {}", m)).dismiss_button("Ok"));
                                            get_input_material(s);
                                        },
                                        Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                                    }
                                },
                                false => {
                                    let new_plan = Forging::new(
                                        planned_date,
                                        machine.unwrap().to_string(),
                                        part_code[0].clone(),
                                        qty
                                    );
        
                                    match Forging::post(&new_plan) {
                                        Ok(0) => s.add_layer(Dialog::info("Check planning again")),
                                        Ok(m) =>{
                                            s.pop_layer();
                                            s.pop_layer();
                                            s.pop_layer();
                                            s.add_layer(Dialog::text(
                                                format!(
                                                    "Plan added successfully. Insert ID: {}. \nRaise Cutting Order for {} parts",
                                                    m,
                                                    (planned_qty - qty) as usize
                                                )
                                            )
                                            .dismiss_button("Ok"));
                                            get_input_material(s);
                                        },
                                        Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
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
            true => s.add_layer(Dialog::new().padding_lrtb(10, 10, 0, 0).content(TextView::new(format!("No Forging Plan created!"))).dismiss_button("Ok")),
            false => {
                s.add_layer(
                    Dialog::new()
                    .title("Forging Plan")
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
                                    .child(TextView::new(format!("Machine")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Part No")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Planned Qty (Nos)")).center().fixed_width(20))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Actual Qty (Nos)")).center().fixed_width(20))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("OK Qty (Nos)")).center().fixed_width(20))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Reject Qty (Nos)")).center().fixed_width(20))
                                    .child(TextView::new(format!("|")).center().fixed_width(3))
                                    .child(TextView::new(format!("Issued Qty (Nos)")).center().fixed_width(20))
                                    .child(TextView::new("Update Production").center().fixed_width(20))
                                );
        
                                let mut count: usize = 0;
                                for forge in forging_list {
                                    count = count + 1;

                                    let enable_button: bool = match &forge[7].parse::<usize>().unwrap() {
                                        0 => true,
                                        _ => false
                                    };

                                    list
                                    .add_child(
                                        "",
                                        LinearLayout::new(Horizontal)
                                        .child(TextView::new(format!("{:?}", count)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&forge[2]).center().fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&forge[3]).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&forge[4]).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&forge[6]).center().fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&forge[7]).center().fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&forge[8]).center().fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&forge[9]).center().fixed_width(20))
                                        .child(TextView::new(format!("|")).center().fixed_width(3))
                                        .child(TextView::new(&forge[10]).center().fixed_width(20))
                                        .child(Button::new_raw(
                                            "        Update       ",
                                            move |s| {
                                                let c_id = &forge[0];
                                                let f_id = &forge[1];
                                                update_forging_status(s, c_id.to_string(), f_id.to_string())
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
            }
        }
    }

    pub fn update_forging_status(s: &mut Cursive, c: String, f: String) {

        s.add_layer(
            Dialog::new()
            .title("Update Cutting Status")
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