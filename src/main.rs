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
        }
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
        //         .subtree(
        //             "User",
        //             menu::MenuTree::new()
        //                 .leaf(
        //                     "New",
        //                     |s| {
        //                         s.add_layer(
        //                             Dialog::new()
        //                                 .title("Create New User")
        //                                 .padding_lrtb(1, 1, 1, 1)
        //                                 .content(
        //                                     ListView::new()
        //                                         .child("Employee ID", EditView::new().with_name("employee_id").fixed_width(30))
        //                                         .child("Email ID", EditView::new().with_name("email").fixed_width(30))
        //                                         .child("Username", EditView::new().with_name("username").fixed_width(30))
        //                                         .child("Password", EditView::new().secret().with_name("password").fixed_width(30))
        //                                         .child("Role", EditView::new().with_name("role").fixed_width(30))
        //                                         .child("Authority", EditView::new().with_name("authority").fixed_width(30))
        //                                 )
        //                                 .button(
        //                                     "Register",
        //                                     |s| {
        //                                         let employee_id = s.call_on_name("employee_id", |v: &mut EditView| {
        //                                             v.get_content()
        //                                         }).unwrap();

        //                                         let email = s.call_on_name("email", |v: &mut EditView| {
        //                                             v.get_content()
        //                                         }).unwrap();

        //                                         let username = s.call_on_name("username", |v: &mut EditView| {
        //                                             v.get_content()
        //                                         }).unwrap();

        //                                         let password = s.call_on_name("password", |v: &mut EditView| {
        //                                             v.get_content()
        //                                         }).unwrap();

        //                                         let role = s.call_on_name("role", |v: &mut EditView| {
        //                                             v.get_content()
        //                                         }).unwrap();

        //                                         let authority = s.call_on_name("authority", |v: &mut EditView| {
        //                                             v.get_content()
        //                                         }).unwrap();

        //                                         let new_user = User::new(
        //                                             employee_id.to_string(),
        //                                             email.to_string(),
        //                                             username.to_string(),
        //                                             password.to_string(),
        //                                             role.to_string(),
        //                                             authority.to_string()
        //                                         );

        //                                         match User::post(new_user) {
        //                                             Ok(_) => s.add_layer(Dialog::text("Authority added successfully").dismiss_button("Ok")),
        //                                             Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
        //                                         };
        //                                     }
        //                                 )
        //                                 .dismiss_button("Cancel")
        //                         )
        //                     }
        //                 )
        //         )
        //         .leaf(
        //             "Quit",
        //             |s| {
                        
        //             }
        //         )
        // )
        // .add_subtree(
        //     "Human Resources",
        //         menu::MenuTree::new()
        //             .subtree(
        //                 "Department",
        //                 menu::MenuTree::new()
        //                     .leaf(
        //                         "New",
        //                         |s| {
        //                             s.add_layer(
        //                                 Dialog::new()
        //                                     .title("Add new department")
        //                                     .padding_lrtb(1, 1, 1, 0)
        //                                     .content(
        //                                         ListView::new()
        //                                             .child("Department Code", EditView::new().with_name("department_code").fixed_width(30))
        //                                             .child("Department Description", EditView::new().with_name("description").fixed_width(30))
        //                                             .child("Department Email ID", EditView::new().with_name("email").fixed_width(30))
        //                                     )
        //                                     .button("Add", |s| {

        //                                         let department_code = s.call_on_name("department_code", |view: &mut EditView| {
        //                                             view.get_content()
        //                                         }).unwrap();
                                                
        //                                         let description = s.call_on_name("description", |view: &mut EditView| {
        //                                             view.get_content()
        //                                         }).unwrap();
                                                
        //                                         let email = s.call_on_name("email", |view: &mut EditView| {
        //                                             view.get_content()
        //                                         }).unwrap();

        //                                         let new = Department::new(
        //                                             department_code.to_string(),
        //                                             description.to_string(),
        //                                             email.to_string()
        //                                         );

        //                                         match Department::post(new) {
        //                                             Ok(_) => s.add_layer(Dialog::text("Department added successfully").dismiss_button("Ok")),
        //                                             Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
        //                                         };
        //                                     })
        //                                     .dismiss_button("Cancel")
        //                             )
        //                         }
        //                     )
        //                     .leaf(
        //                         "Find by Department Code",
        //                         |s| {
        //                             s.add_layer(
        //                                 Dialog::new()
        //                                     .title("Find the department")
        //                                     .padding_lrtb(1, 1, 1, 0)
        //                                     .content(
        //                                         EditView::new()
        //                                             .with_name("search_query")
        //                                             .fixed_width(30)
        //                                     )
        //                                     .button("Search", |s| {
        //                                         let search_query = s.call_on_name("search_query",|view: &mut EditView| {
        //                                             view.get_content()
        //                                         }).unwrap();

        //                                         let result = Department::find_by_dept_code(search_query.to_string()).unwrap();

        //                                         if result.len() != 0 {
        //                                             s.add_layer(Dialog::info(result[0][2].to_string()));
        //                                         } else {
        //                                             s.add_layer(Dialog::info("Department does not exist!"));
        //                                         }
        //                                     })
        //                                     .dismiss_button("Cancel")
        //                             )
        //                         }
        //                     )
        //                     .leaf(
        //                         "Edit",
        //                         |s| {}
        //                     )
        //                     .leaf(
        //                         "Delete",
        //                         |s| {}
        //                     )
        //             )
        //             .subtree(
        //                 "Employee",
        //                 menu::MenuTree::new()
        //                     .leaf(
        //                         "New",
        //                         |s| {
        //                             s.add_layer(
        //                                 Dialog::new()
        //                                     .title("Add new employee")
        //                                     .padding_lrtb(1, 1, 1, 0)
        //                                     .content(
        //                                         ListView::new()
        //                                             .child("Department Code", EditView::new().with_name("department_code").fixed_width(30))
        //                                             .child("Department Description", EditView::new().with_name("description").fixed_width(30))
        //                                             .child("Department Email ID", EditView::new().with_name("email").fixed_width(30))
        //                                     )
        //                                     .button("Add", |s| {

        //                                         let department_code = s.call_on_name("department_code", |view: &mut EditView| {
        //                                             view.get_content()
        //                                         }).unwrap();
                                                
        //                                         let description = s.call_on_name("description", |view: &mut EditView| {
        //                                             view.get_content()
        //                                         }).unwrap();
                                                
        //                                         let email = s.call_on_name("email", |view: &mut EditView| {
        //                                             view.get_content()
        //                                         }).unwrap();

        //                                         // let new = Employee::new(
        //                                         //     department_code.to_string(),
        //                                         //     description.to_string(),
        //                                         //     email.to_string()
        //                                         // );

        //                                         // match Employee::post(new) {
        //                                         //     Ok(_) => s.add_layer(Dialog::text("New Employee added successfully").button("Ok", |s| {  })),
        //                                         //     Err(_) => s.add_layer(Dialog::text("Error encountered").button("Ok", |s| {  }))
        //                                         // };

        //                                     })
        //                                     .dismiss_button("Cancel")
        //                             )
        //                         }
        //                     )
        //             )
        // )
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
    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    siv.add_layer(Dialog::text("Hit <Esc> to show the menu!"));

    siv.run();
}