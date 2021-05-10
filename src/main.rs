mod apis;
mod frontend;

use cursive::{
    Cursive,
    traits::*,
    CursiveExt,
    event::Key,
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
    raw_material::{
        steel::steel::Steel,
        grades::grades::Grades,
        section::section::Section,
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
                                                    Ok(_) => s.add_layer(Dialog::text("Role added successfully").button("Ok", |s| { })),
                                                    Err(_) => s.add_layer(Dialog::text("Error encountered").dismiss_button("Ok"))
                                                };
                                            }
                                        )
                                        .dismiss_button("Cancel")
                                )
                            }
                        )
                        .leaf(
                            "Assign Activities",
                            |s| {
                                let v = Authorities::get().unwrap();

                                let r = Roles::get().unwrap();

                                match &v.is_empty() {
                                    true => {
                                        s.add_layer(Dialog::info("No activity created!"))
                                    },
                                    false => {
                                        s.add_layer(
                                            Dialog::new()
                                                .title("Assign Activities")
                                                .padding_lrtb(1, 1, 1, 1)
                                                .content(
                                                    ListView::new()
                                                        .child(
                                                            "Role",
                                                            SelectView::<String>::new()
                                                            .popup()
                                                            .h_align(HAlign::Center)
                                                            .autojump()
                                                            .with_all_str(r)
                                                            .on_select(|s, item| {
                                                                println!("{}", &item);
                                                            }
                                                            )
                                                            .with_name("role")
                                                        )
                                                        .child(
                                                            "Activity",
                                                            SelectView::<String>::new()
                                                            .popup()
                                                            .h_align(HAlign::Center)
                                                            .autojump()
                                                            .with_all_str(v)
                                                            .on_select(|s, item| {
                                                                println!("{}", &item);
                                                            }
                                                            )
                                                            .with_name("activity")
                                                        )
                                                )
                                                .button(
                                                    "Add",
                                                    |s| {
                                                        let rol = s.call_on_name("role", |v: &mut SelectView| {
                                                            v.selection()
                                                        }).unwrap();

                                                        let sel = s.call_on_name("activity", |v: &mut SelectView| {
                                                            v.selection()
                                                        }).unwrap();

                                                        Authorities::assign(rol.unwrap().to_string(), sel.unwrap().to_string()).unwrap();
                                                    }
                                                )
                                        )
                                    }
                                }
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
                                                    Ok(_) => s.add_layer(Dialog::text("Authority added successfully").dismiss_button("Ok")),
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

                                                let new_user = User::new(
                                                    employee_id.to_string(),
                                                    email.to_string(),
                                                    username.to_string(),
                                                    password.to_string(),
                                                    role.to_string(),
                                                    authority.to_string()
                                                );

                                                match User::post(new_user) {
                                                    Ok(_) => s.add_layer(Dialog::text("Authority added successfully").button("Ok", |s| { })),
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
                                                    Ok(_) => s.add_layer(Dialog::text("Department added successfully").button("Ok", |s| {  })),
                                                    Err(_) => s.add_layer(Dialog::text("Error encountered").button("Ok", |s| {  }))
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
                                                //     Ok(_) => s.add_layer(Dialog::text("New Employee added successfully").button("Ok", |s| {  })),
                                                //     Err(_) => s.add_layer(Dialog::text("Error encountered").button("Ok", |s| {  }))
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
                    "Steel",
                    menu::MenuTree::new()
                        .leaf(
                            "Add Grades",
                            |s| {
                                s.add_layer(
                                    Dialog::new()
                                        .title("Add new grade")
                                        .padding_lrtb(1,1,1,0)
                                        .content(
                                            ListView::new()
                                                .child("Grade", EditView::new().with_name("grade").fixed_width(30))
                                        )
                                        .button(
                                            "Add",
                                            |s| {
                                                let grade = s.call_on_name("grade", |v: &mut EditView| {
                                                    v.get_content()
                                                }).unwrap();

                                                let new_grade = Grades::new(grade.to_string());

                                                match Grades::post(new_grade) {
                                                    Ok(_) => s.add_layer(Dialog::text("Grade added successfully").dismiss_button("Ok")),
                                                    Err(_) => s.add_layer(Dialog::text("Error encountered").dismiss_button("Ok"))
                                                };
                                            }
                                        )
                                        .dismiss_button("Cancel")
                                )
                            }
                        )
                        .leaf(
                            "Add Section",
                            |s| {
                                s.add_layer(
                                    Dialog::new()
                                    .title("Add Section")
                                    .padding_lrtb(1,1,1,0)
                                    .content(
                                        ListView::new()
                                            .child("Size", EditView::new().with_name("sec_size").fixed_width(30))
                                            .child(
                                                "Type",
                                                SelectView::new()
                                                    .popup()
                                                    .h_align(HAlign::Center)
                                                    .autojump()
                                                    .with_all_str(&["RCS".to_string(), "DIA".to_string()])
                                                    .on_select(|s, item| {
                                                        println!("{}", item);
                                                    })
                                                    .with_name("sec_type")
                                            )
                                    )
                                    .button(
                                        "Add",
                                        |s| {
                                            let sec_size = s.call_on_name("sec_size", |v: &mut EditView| {
                                                v.get_content()
                                            }).unwrap();
                                            let sec_type = s.call_on_name("sec_type", |v: &mut SelectView| {
                                                v.selection()
                                            }).unwrap();

                                            let new_section = Section::new(sec_size.parse::<u16>().unwrap(), sec_type.unwrap().to_string());

                                            match Section::post(new_section) {
                                                Ok(_) => s.add_layer(Dialog::text("Section added successfully").dismiss_button("Ok")),
                                                Err(_) => s.add_layer(Dialog::text("Error encountered").dismiss_button("Ok"))
                                            };
                                        }
                                    )
                                    .dismiss_button("Cancel")
                                )
                            }
                        )
                        .leaf(
                            "Create Steel",
                            |s| {
                                let g = Grades::get().unwrap();

                                let sec = &Section::get().unwrap();

                                let mut v: Vec<String> = Vec::new();

                                for i in 0..sec.len() {
                                    let section = format!("{} {}", sec[i].0, sec[i].1);

                                    v.push(section);
                                }

                                match &g.is_empty() {
                                    true => {
                                        s.add_layer(Dialog::info("No grades created!"))
                                    },
                                    false => {
                                        s.add_layer(
                                            Dialog::new()
                                                .title("Assign Steel")
                                                .padding_lrtb(1, 1, 1, 1)
                                                .content(
                                                    ListView::new()
                                                        .child(
                                                            "Grades",
                                                            SelectView::<String>::new()
                                                            .popup()
                                                            .h_align(HAlign::Center)
                                                            .autojump()
                                                            .with_all_str(g)
                                                            .on_select(|s, item| {
                                                                println!("{}", &item);
                                                            }
                                                            )
                                                            .with_name("grades")
                                                        )
                                                        .child(
                                                            "Section",
                                                            SelectView::<String>::new()
                                                            .popup()
                                                            .h_align(HAlign::Center)
                                                            .autojump()
                                                            .with_all_str(v)
                                                            .on_select(|s, item| {
                                                                println!("{}", &item);
                                                            }
                                                            )
                                                            .with_name("section")
                                                        )
                                                )
                                                .button(
                                                    "Add",
                                                    |s| {
                                                        let grd = s.call_on_name("grades", |v: &mut SelectView| {
                                                            v.selection()
                                                        }).unwrap();

                                                        let sec = s.call_on_name("section", |v: &mut SelectView| {
                                                            v.selection()
                                                        }).unwrap();

                                                        Steel::assign(grd, sec).unwrap();
                                                    }
                                                )
                                        )
                                    }
                                }
                            }
                        )
                )
        );
    siv.add_global_callback(Key::Esc, |s| s.select_menubar());

    siv.add_layer(Dialog::text("Hit <Esc> to show the menu!"));

    siv.run();
}