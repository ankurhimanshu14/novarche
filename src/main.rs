mod apis;

use cursive::{
    Cursive,
    traits::*,
    CursiveExt,
    menu,
    view::{ Nameable },
    align::{ HAlign },
    views::{ Menubar, Dialog, EditView, ListView, SelectView, TextView },
};

use apis::human_resources::department::department::Department;
use apis::raw_material::steel::steel::Steel;

fn main() {
    let mut siv = Cursive::new();

    siv.add_layer(Menubar::new());

    siv.menubar()
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

                                                Department::post(new);

                                                s.quit();
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

                                                let new = Department::new(
                                                    department_code.to_string(),
                                                    description.to_string(),
                                                    email.to_string()
                                                );

                                                Department::post(new);

                                                s.quit();
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
                                                .child(
                                                    "Section Type",
                                                    SelectView::new()
                                                        .h_align(HAlign::Center)
                                                        .popup()
                                                        .autojump()
                                                        .item("Select..", 0)
                                                        .item("RCS", 1)
                                                        .item("DIA", 2)
                                                        .on_select(|_, item| {

                                                            TextView::new("").with_name("section_type");

                                                            match *item {
                                                                1 => "RCS",
                                                                2 => "DIA",
                                                                _ => unreachable!("no such item"),
                                                            };
                                                        }
                                                        )
                                                )
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
                                                view.selection();
                                            });

                                            println!("{:?}", &section_type);

                                            // let new = Steel::new(
                                            //     grade.to_string(),
                                            //     item_code.to_string(),
                                            //     section_size.parse::<usize>().unwrap(),
                                            //     section_type.unwrap()
                                            // );

                                            // Steel::post(new);

                                            s.quit();
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