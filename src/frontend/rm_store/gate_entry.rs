pub mod gate_entry {

    use chrono::NaiveDate;

    use cursive::{
        Cursive,
        traits::*,
        CursiveExt,
        event::Key,
        menu,
        view::{ Nameable, Resizable },
        align::{ HAlign, VAlign },
        views::{ Menubar, Dialog, EditView, ListView, SelectView, TextView, TextArea },
    };

    use crate::apis::rm_store:: {
        gate_entry::gate_entry::GateEntry,
        party::party::Party
    };

    pub fn create_grn(s: &mut Cursive) {

        let p = Party::get().unwrap();

        s.add_layer(
            Dialog::new()
            .title("Gate Entry")
            .padding_lrtb(1, 1, 1, 0)
            .content(
                ListView::new()
                .child("Challan No", EditView::new().with_name("challan_no").fixed_width(30).min_height(2))
                .child("Challan Date", EditView::new().with_name("challan_date").fixed_width(30).min_height(2))
                .child("Item Code", EditView::new().with_name("item_code").fixed_width(30).min_height(2))
                .child("Item Description", TextArea::new().content("Write description here...").with_name("item_description").fixed_width(30).min_height(5))
                .child(
                    "Party Name",
                    SelectView::new()
                    .popup()
                    .autojump()
                    .with_all_str(p)
                    .on_select(|s, item| {
                        println!("{}", &item);
                    }
                    )
                    .with_name("party_name")
                    .fixed_width(30)
                    .min_height(2)
                )
                .child("Heat No", EditView::new().with_name("heat_no").fixed_width(30).min_height(2))
                .child("Received Quantity", EditView::new().with_name("received_qty").fixed_width(30).min_height(2))
                .child("U. O. M.", EditView::new().with_name("uom").fixed_width(30).min_height(2))
                .child("Unit Cost", EditView::new().with_name("unit_cost").fixed_width(30).min_height(2))
            )
            .button(
                "Add",
                |s| {
                    let challan_no = s.call_on_name("challan_no", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let challan_date = s.call_on_name("challan_date", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let challan_date = NaiveDate::parse_from_str(&challan_date, "%d-%m-%Y").unwrap();

                    let item_code = s.call_on_name("item_code", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let item_description = s.call_on_name("item_description", |v: &mut TextArea| {
                        v.get_content().to_string()
                    }).unwrap();

                    let party_name = s.call_on_name("party_name", |v: &mut SelectView| {
                        v.selection()
                    }).unwrap();

                    let party_code = Party::get_party_code(&party_name.unwrap().to_string());

                    let heat_no = s.call_on_name("heat_no", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let received_qty = s.call_on_name("received_qty", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let uom = s.call_on_name("uom", |v: &mut EditView|{
                        v.get_content()
                    }).unwrap();

                    let unit_cost = s.call_on_name("unit_cost", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    match GateEntry::new(
                        challan_no.parse::<usize>().unwrap(),
                        challan_date,
                        item_code.to_string(),
                        item_description.to_string(),
                        party_code[0].clone(),
                        heat_no.to_string(),
                        received_qty.parse::<f64>().unwrap(),
                        uom.to_string(),
                        Some(unit_cost.parse::<f64>().unwrap())
                    ).post() {
                        Ok(_) =>{
                            s.pop_layer();
                            s.add_layer(Dialog::text("Gate Entry added successfully").dismiss_button("Ok"))
                        },
                        Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                    }
                }
            )
            .dismiss_button("Cancel")
        )
    }

    pub fn assign_parts(s: &mut Cursive) {

        let h = GateEntry::get_heat_no_list().unwrap();

        s.add_layer(
            Dialog::new()
            .title("Assign approved parts")
            .padding_lrtb(1, 1, 1, 0)
            .content(
                ListView::new()
                .child(
                    "Heat No",
                    SelectView::new()
                    .popup()
                    .autojump()
                    .with_all_str(h)
                    .on_select(|s, item| {
                        println!("{:?}", &item);
                    })
                    .with_name("heat_no")
                    .fixed_width(30)
                    .min_height(2))
                .with(|list| {
                    for i in 1..20 {
                        list
                        .add_child(&format!("Part {}", &i),
                        EditView::new()
                        .with_name(&format!("part_no_{}", &i))
                        .fixed_width(30)
                        .min_height(2)
                        )
                    }
                })
                .scrollable()
            )
            .button(
                "Add",
                |s| {
                    let heat_no = s.call_on_name("heat_no", |v: &mut SelectView| {
                        v.selection()
                    }).unwrap();

                    let mut v_part: Vec<usize> = Vec::new();

                    for i in 1..20 {
                        let part_no = s.call_on_name(&format!("part_no_{}", i), |v: &mut EditView| {
                            v.get_content()
                        }).unwrap();
                        match part_no.to_string().len() {
                            0 => {},
                            _ => v_part.push(part_no.to_string().parse::<usize>().unwrap())
                        }
                    }

                    match GateEntry::assign_approvals(heat_no.unwrap().to_string(), v_part) {
                        Ok(_) =>{
                            s.pop_layer();
                            s.add_layer(Dialog::text("Gate Entry added successfully").dismiss_button("Ok"))
                        },
                        Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                    }
                }
            )
            .dismiss_button("Cancel")
        )
    }
}