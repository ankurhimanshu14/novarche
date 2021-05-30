pub mod inventory {

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

    use crate::apis::rm_store::inventory::inventory::Inventory;

    pub fn inventory(s: &mut Cursive) {

        let inv = Inventory::inventory().unwrap();

        match inv.is_empty() {
            true => s.add_layer(Dialog::new().padding_lrtb(10, 10, 0, 0).content(TextView::new(format!("No Inventory Available!"))).dismiss_button("Ok")),
            false => {
                s.add_layer(
                    Dialog::new()
                    .title("Inventory")
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
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Heat No")).center().fixed_width(15))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Grade")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Size")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Section")).center().fixed_width(10))
                                    .child(TextView::new(format!("|")).center().fixed_width(1))
                                    .child(TextView::new(format!("Available Quantity (Kgs)")).center().fixed_width(20))
                                );
        
                                let mut count: usize = 0;
                                for i in inv {
                                    count = count + 1;

                                    list
                                    .add_child(
                                        "",
                                        LinearLayout::new(Horizontal)
                                        .child(TextView::new(format!("{0}", count)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", i.heat_no)).center().fixed_width(15))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", i.grade)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", i.size)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", i.section)).center().fixed_width(10))
                                        .child(TextView::new(format!("|")).center().fixed_width(1))
                                        .child(TextView::new(format!("{0}", i.avail_qty)).center().fixed_width(20))
                                    )
                                }
                            }
                        )
                        .min_height(10)
                        .scrollable()
        
                    )
                    .dismiss_button("Close")
                )
            }
        }
    }
}