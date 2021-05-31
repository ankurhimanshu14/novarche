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

    use crate::frontend::production::cutting::cutting::display_cutting_heat;

    pub fn forging_plan(s: &mut Cursive) {
        s.add_layer(
            Dialog::new()
            .title("Create Forging Plan")
            .padding_lrtb(1, 1, 1, 1)
            .content(
                ListView::new()
                .child("Planning Date", EditView::new().with_name("planned_date").fixed_width(30))
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
                .child("Part No", EditView::new().with_name("part_no").fixed_width(30))
            )
            .button(
                "Select Cutting",
                |s| {
                    let planned_date = s.call_on_name("planned_date", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let planned_date = NaiveDate::parse_from_str(&planned_date.to_string(), "%d-%m-%Y").unwrap();

                    let machine = s.call_on_name("machine", |v: &mut SelectView| {
                        v.selection()
                    }).unwrap();

                    let p_no = s.call_on_name("part_no", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let part_no = p_no.parse::<usize>().unwrap();

                    let part_code = Part::find_part_code(part_no);

                    match part_code.len() {
                        0 => s.add_layer(Dialog::info("Part not defined")),
                        1 => {
                            display_cutting_heat(s, part_no);
                        },
                        _ => s.add_layer(Dialog::info("Error is fetching part list"))
                    }
                }
            )
            .dismiss_button("Ok")
        )
    }
}