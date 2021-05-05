mod apis;

use cursive::{
    Cursive,
    traits::*,
    CursiveExt,
    menu,
    view::{ Nameable, Resizable },
    align::{ HAlign },
    views::{ Menubar, Dialog, EditView, ListView, SelectView, TextView },
};

use apis::{
    human_resources::{
        employee::employee::Employee,
        department::department::Department,
    },
    admin::{
        roles::roles::Roles,
        authorities::authorities::Authorities,
        user::user::User,
    },
    raw_material::steel::steel::Steel,
};

fn main() {
    let mut siv = Cursive::new();

    siv.add_layer(Menubar::new());

    siv.menubar()
        .add_subtree(
            "Administration",
            menu::MenuTree::new()
                .subtree(
                    "Roles",
                    menu::MenuTree::new()
                        .leaf(
                            "New",
                            |s| {
                                s.add_layer(
                                    Dialog::new()
                                        .title("Create Roles")
                                        .padding_lrtb(1, 1, 1, 0)
                                        .content(
                                            ListView::new()
                                                .child("Roles Name", EditView::new().with_name("roles_name").fixed_width(30))
                                        )
                                        .button(
                                            "Add",
                                            |s| {

                                                let roles_name = s.call_on_name("roles_name", |v: &mut EditView| {
                                                    v.get_content()
                                                }).unwrap();

                                                let new_role = Roles::new(roles_name.to_string());

                                                match Roles::post(new_role) {
                                                    Ok(_) => s.add_layer(Dialog::text("Role added successfully").button("Ok", |s| { s.quit()})),
                                                    Err(_) => s.add_layer(Dialog::text("Error encountered").dismiss_button("Ok"))
                                                };
                                            }
                                        )
                                        .dismiss_button("Cancel")
                                )
                            }
                        )
                )
                .subtree(
                    "Authorities",
                    menu::MenuTree::new()
                        .leaf(
                            "New",
                            |s| {
                                s.add_layer(
                                    Dialog::new()
                                        .title("Create Authorities")
                                        .padding_lrtb(1, 1, 1, 0)
                                        .content(
                                            ListView::new()
                                                .child("Activity Name", EditView::new().with_name("activity").fixed_width(30))
                                        )
                                        .button(
                                            "Add",
                                            |s| {

                                                let activity = s.call_on_name("activity", |v: &mut EditView| {
                                                    v.get_content()
                                                }).unwrap();

                                                let new_authority = Authorities::new(activity.to_string());

                                                match Authorities::post(new_authority) {
                                                    Ok(_) => s.add_layer(Dialog::text("Authority added successfully").button("Ok", |s| { s.quit()})),
                                                    Err(_) => s.add_layer(Dialog::text("Error encountered").dismiss_button("Ok"))
                                                };
                                            }
                                        )
                                        .dismiss_button("Cancel")
                                )
                            }
                        )
                )
                .subtree(
                    "User",
                    menu::MenuTree::new()
                        .leaf(
                            "New",
                            |s| {
                                s.add_layer(
                                    Dialog::new()
                                        .title("Create New User")
                                        .padding_lrtb(1, 1, 1, 1)
                                        .content(
                                            ListView::new()
                                                .child("Employee ID", EditView::new().with_name("employee_id").fixed_width(30))
                                                .child("Email ID", EditView::new().with_name("email").fixed_width(30))
                                                .child("Username", EditView::new().with_name("username").fixed_width(30))
                                                .child("Password", EditView::new().secret().with_name("password").fixed_width(30))
                                                .child("Role", EditView::new().with_name("role").fixed_width(30))
                                                .child("Authority", EditView::new().with_name("authority").fixed_width(30))
                                        )
                                        .button(
                                            "Register",
                                            |s| {
                                                let employee_id = s.call_on_name("employee_id", |v: &mut EditView| {
                                                    v.get_content()
                                                }).unwrap();

                                                let email = s.call_on_name("email", |v: &mut EditView| {
                                                    v.get_content()
                                                }).unwrap();

                                                let username = s.call_on_name("username", |v: &mut EditView| {
                                                    v.get_content()
                                                }).unwrap();

                                                let password = s.call_on_name("password", |v: &mut EditView| {
                                                    v.get_content()
                                                }).unwrap();

                                                let role = s.call_on_name("role", |v: &mut EditView| {
                                                    v.get_content()
                                                }).unwrap();

                                                let authority = s.call_on_name("authority", |v: &mut EditView| {
                                                    v.get_content()
                                                }).unwrap();

                                                let new_user = User::new(employee_id.to_string(), email.to_string(), username.to_string(), password.to_string(), role.to_string(), authority.to_string());

                                                match User::post(new_user) {
                                                    Ok(_) => s.add_layer(Dialog::text("Authority added successfully").button("Ok", |s| { s.quit()})),
                                                    Err(_) => s.add_layer(Dialog::text("Error encountered").dismiss_button("Ok"))
                                                };
                                            }
                                        )
                                        .dismiss_button("Cancel")
                                )
                            }
                        )
                )
                .leaf(
                    "Quit",
                    |s| {
                        s.quit()
                    }
                )
        )
        .add_subtree(
            "Human Resources",
                menu::MenuTree::new()
                    .subtree(
                        "Department",
                        menu::MenuTree::new()
                            .leaf(
                                "New",
                                |s| {
                                    s.add_layer(
                                        Dialog::new()
                                            .title("Add new department")
                                            .padding_lrtb(1, 1, 1, 0)
                                            .content(
                                                ListView::new()
                                                    .child("Department Code", EditView::new().with_name("department_code").fixed_width(30))
                                                    .child("Department Description", EditView::new().with_name("description").fixed_width(30))
                                                    .child("Department Email ID", EditView::new().with_name("email").fixed_width(30))
                                            )
                                            .button("Add", |s| {

                                                let department_code = s.call_on_name("department_code", |view: &mut EditView| {
                                                    view.get_content()
                                                }).unwrap();
                                                
                                                let description = s.call_on_name("description", |view: &mut EditView| {
                                                    view.get_content()
                                                }).unwrap();
                                                
                                                let email = s.call_on_name("email", |view: &mut EditView| {
                                                    view.get_content()
                                                }).unwrap();

                                                let new = Department::new(
                                                    department_code.to_string(),
                                                    description.to_string(),
                                                    email.to_string()
                                                );

                                                match Department::post(new) {
                                                    Ok(_) => s.add_layer(Dialog::text("Department added successfully").button("Ok", |s| { s.quit() })),
                                                    Err(_) => s.add_layer(Dialog::text("Error encountered").button("Ok", |s| { s.quit() }))
                                                };
                                            })
                                            .dismiss_button("Cancel")
                                    )
                                }
                            )
                            .leaf(
                                "Find by Department Code",
                                |s| {
                                    s.add_layer(
                                        Dialog::new()
                                            .title("Find the department")
                                            .padding_lrtb(1, 1, 1, 0)
                                            .content(
                                                EditView::new()
                                                    .with_name("search_query")
                                                    .fixed_width(30)
                                            )
                                            .button("Search", |s| {
                                                let search_query = s.call_on_name("search_query",|view: &mut EditView| {
                                                    view.get_content()
                                                }).unwrap();

                                                let result = Department::find_by_dept_code(search_query.to_string()).unwrap();

                                                if result.len() != 0 {
                                                    s.add_layer(Dialog::info(result[0][2].to_string()));
                                                } else {
                                                    s.add_layer(Dialog::info("Department does not exist!"));
                                                }
                                            })
                                            .dismiss_button("Cancel")
                                    )
                                }
                            )
                            .leaf(
                                "Edit",
                                |s| {}
                            )
                            .leaf(
                                "Delete",
                                |s| {}
                            )
                    )
                    .subtree(
                        "Employee",
                        menu::MenuTree::new()
                            .leaf(
                                "New",
                                |s| {
                                    s.add_layer(
                                        Dialog::new()
                                            .title("Add new employee")
                                            .padding_lrtb(1, 1, 1, 0)
                                            .content(
                                                ListView::new()
                                                    .child("Department Code", EditView::new().with_name("department_code").fixed_width(30))
                                                    .child("Department Description", EditView::new().with_name("description").fixed_width(30))
                                                    .child("Department Email ID", EditView::new().with_name("email").fixed_width(30))
                                            )
                                            .button("Add", |s| {

                                                let department_code = s.call_on_name("department_code", |view: &mut EditView| {
                                                    view.get_content()
                                                }).unwrap();
                                                
                                                let description = s.call_on_name("description", |view: &mut EditView| {
                                                    view.get_content()
                                                }).unwrap();
                                                
                                                let email = s.call_on_name("email", |view: &mut EditView| {
                                                    view.get_content()
                                                }).unwrap();

                                                // let new = Employee::new(
                                                //     department_code.to_string(),
                                                //     description.to_string(),
                                                //     email.to_string()
                                                // );

                                                // match Employee::post(new) {
                                                //     Ok(_) => s.add_layer(Dialog::text("New Employee added successfully").button("Ok", |s| { s.quit() })),
                                                //     Err(_) => s.add_layer(Dialog::text("Error encountered").button("Ok", |s| { s.quit() }))
                                                // };

                                            })
                                            .dismiss_button("Cancel")
                                    )
                                }
                            )
                    )
        )
        .add_subtree(
            "Store",
            menu::MenuTree::new()
                .subtree(
                    "Raw Material",
                    menu::MenuTree::new()
                        .leaf(
                            "New",
                            |s| {}
                        )
                )
                .subtree(
                    "General Store",
                    menu::MenuTree::new()
                        .leaf(
                            "New",
                            |s| {}
                        )
                )
        )
        .add_subtree(
            "Item Master",
            menu::MenuTree::new()
                .subtree(
                    "Steels",
                    menu::MenuTree::new()
                        .leaf(
                            "New",
                            |s| {
                                s.add_layer(
                                    Dialog::new()
                                        .title("Add new steel")
                                        .padding_lrtb(1, 1, 1, 0)
                                        .content(                                            
                                            ListView::new()
                                                .child("Grade", EditView::new().with_name("grade").fixed_width(30))
                                                .child("Item Code", EditView::new().with_name("item_code").fixed_width(30))
                                                .child("Section Size", EditView::new().with_name("section_size").fixed_width(30))
                                                .child("Section Type",EditView::new().with_name("section_type").fixed_width(30))
                                        )
                                        .button("Add", |s| {

                                            let grade = s.call_on_name("grade", |view: &mut EditView| {
                                                view.get_content()
                                            }).unwrap();
                                            
                                            let item_code = s.call_on_name("item_code", |view: &mut EditView| {
                                                view.get_content()
                                            }).unwrap();
                                            
                                            let section_size = s.call_on_name("section_size", |view: &mut EditView| {
                                                view.get_content()
                                            }).unwrap();

                                            let section_type = s.call_on_name("section_type", |view: &mut SelectView| {
                                                SelectView::new()
                                                    .h_align(HAlign::Center)
                                                    .popup()
                                                    .autojump()
                                                    .item_str("Select..")
                                                    .item_str("RCS")
                                                    .item_str("DIA");
                                                view.selection();
                                            }).unwrap();

                                            println!("{:?}", &section_type);

                                            // let new = Steel::new(
                                            //     grade.to_string(),
                                            //     item_code.to_string(),
                                            //     section_size.parse::<usize>().unwrap(),
                                            //     section_type
                                            // );

                                            // match Steel::post(new) {
                                            //     Ok(_) => s.add_layer(Dialog::text("New Steel added successfully").button("Ok", |s| { s.quit() })),
                                            //     Err(_) => s.add_layer(Dialog::text("Error encountered").button("Ok", |s| { s.quit() }))
                                            // };
                                        })
                                        .dismiss_button("Cancel")
                                    )
                            }
                        )
                        )
                );

    siv.select_menubar();

    siv.run();
}