mod apis;
mod frontend;

use cursive::{
    Cursive,
    traits::*,
    CursiveExt,
    event::Key,
    menu,
    view::{ Nameable, Resizable },
    align::{ HAlign, VAlign },
    views::{ Menubar, Dialog, EditView, ListView, SelectView, TextView },
};

use frontend::{
    admin::{
        roles::roles::{
            create_roles,
            delete_roles
        },
        authority::authority::{
            create_authority,
            assign_authority
        },
        user_signup::user_signup::{
            create_user,
        },
        authenticate::authenticate::sign_in
    },
    human_resources::{
        employee::employee::create_employee
    }
};

fn main() {

    let mut siv = Cursive::new();

    siv.add_layer(Menubar::new());

    siv.menubar()
        .add_subtree(
            "File",
            menu::MenuTree::new()
                .leaf(
                    "Exit",
                    |s| { s.quit() }
                )
        )
        .add_subtree(
            "Administration",
            menu::MenuTree::new()
                .subtree(
                    "Roles",
                    menu::MenuTree::new()
                        .leaf(
                            "New",
                            |s| { create_roles(s) }
                        )
                        .leaf(
                            "Delete",
                            |s| { delete_roles(s) }
                        )
                    )
                .subtree(
                    "Authorities",
                    menu::MenuTree::new()
                        .leaf(
                            "New",
                            |s| { create_authority(s) }
                        )
                        .leaf(
                            "Assign Activities",
                            |s| { assign_authority(s) }
                        )
                )
                .subtree(
                    "User",
                    menu::MenuTree::new()
                        .leaf(
                            "Sign Up",
                            |s| { create_user(s) }
                        )
                )
        )
        .add_subtree(
            "Human Resources",
            menu::MenuTree::new()
                .subtree(
                    "Employees",
                    menu::MenuTree::new()
                        .leaf(
                            "New Joining",
                            |s| { create_employee(s) }
                        )
                )
        );
        
        // .add_subtree(
        //     "Store",
        //     menu::MenuTree::new()
        //         .subtree(
        //             "Raw Material",
        //             menu::MenuTree::new()
        //                 .leaf(
        //                     "New",
        //                     |s| {}
        //                 )
        //         )
        //         .subtree(
        //             "General Store",
        //             menu::MenuTree::new()
        //                 .leaf(
        //                     "New",
        //                     |s| {}
        //                 )
        //         )
        // )
        // .add_subtree(
        //     "Engineering",
        //     menu::MenuTree::new()
        //         .subtree(
        //             "Part Master",
        //             menu::MenuTree::new()
        //                 .leaf(
        //                     "New",
        //                     |s| {
        //                         let grd = Grades::get().unwrap();

        //                         s.add_layer(
        //                             Dialog::new()
        //                                 .title("Add new part")
        //                                 .padding_lrtb(1,1,1,0)
        //                                 .content(
        //                                     ListView::new()
        //                                         .child("Part No", EditView::new().with_name("part_no").fixed_width(30))
        //                                         .child("Part Name", EditView::new().with_name("part_name").fixed_width(30))
        //                                         .child(
        //                                             "Material Grade",
        //                                             SelectView::new()
        //                                                 .popup()
        //                                                 .v_align(VAlign::Center)
        //                                                 .autojump()
        //                                                 .with_all_str(grd)
        //                                                 .on_select(|s, item| {
        //                                                     println!("{}", &item)
        //                                                 })
        //                                                 .with_name("grade")
        //                                                 .fixed_width(30))
        //                                         .child("Forging Weight", EditView::new().with_name("forging_wt").fixed_width(30))
        //                                         .child("Cut Weight", EditView::new().with_name("cut_wt").fixed_width(30))
        //                                         .child("Delivery Condition", EditView::new().with_name("del_cond").fixed_width(30))
        //                                         .child("Drawing Revision No", EditView::new().with_name("drawing_rev_no").fixed_width(30))
        //                                         .child("Drawing Revision Date", EditView::new().with_name("drawing_rev_date").fixed_width(30))
        //                                 )
        //                                 .button(
        //                                     "Add",
        //                                     |s| {
        //                                         let part_no = s.call_on_name("part_no", |v: &mut EditView| {
        //                                             v.get_content()
        //                                         }).unwrap();
        //                                         let part_name = s.call_on_name("part_name", |v: &mut EditView| {
        //                                             v.get_content()
        //                                         }).unwrap();
        //                                         let grade = s.call_on_name("grade", |v: &mut SelectView| {
        //                                             v.selection()
        //                                         }).unwrap();
        //                                         let forging_wt = s.call_on_name("forging_wt", |v: &mut EditView| {
        //                                             v.get_content()
        //                                         }).unwrap();
        //                                         let cut_wt = s.call_on_name("cut_wt", |v: &mut EditView| {
        //                                             v.get_content()
        //                                         }).unwrap();
        //                                         let del_cond = s.call_on_name("del_cond", |v: &mut EditView| {
        //                                             v.get_content()
        //                                         }).unwrap();
        //                                         let drawing_rev_no = s.call_on_name("drawing_rev_no", |v: &mut EditView| {
        //                                             v.get_content()
        //                                         }).unwrap();
        //                                         let drawing_rev_date = s.call_on_name("drawing_rev_date", |v: &mut EditView| {
        //                                             v.get_content()
        //                                         }).unwrap();

        //                                         match Part::new(
        //                                             part_no.parse::<usize>().unwrap(),
        //                                             part_name.to_string(),
        //                                             grade.unwrap().to_string(),
        //                                             forging_wt.parse::<f32>().unwrap(),
        //                                             cut_wt.parse::<f32>().unwrap(),
        //                                             del_cond.to_string(),
        //                                             drawing_rev_no.to_string(),
        //                                             drawing_rev_date.to_string()
        //                                         ).post() {
        //                                             Ok(_) => s.add_layer(Dialog::text("New Part added successfully").dismiss_button("Ok")),
        //                                             Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
        //                                         }

        //                                     }
        //                                 )
        //                                 .dismiss_button("Cancel")
        //                         )
        //                     }
        //                 )
        //         )
        // );

    sign_in(&mut siv);

    siv.run();
}