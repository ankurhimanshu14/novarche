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
            change_password
        },
        authenticate::authenticate::sign_in
    },
    human_resources::{
        employee::employee::{
            create_employee,
            update_dept_code
        }
    },
    engineering::{
        parts::parts::create_parts
    },
    raw_material::{
        grades::grades::create_grades
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
                        .leaf(
                            "Change Password",
                            |s| { change_password(s) }
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
                        .leaf(
                            "Change Department",
                            |s|{ update_dept_code(s) }
                        )
                )
        )
        .add_subtree(
            "Engineering",
            menu::MenuTree::new()
            .subtree(
                "Parts",
                menu::MenuTree::new()
                .leaf(
                    "New Part",
                    |s|{ create_parts(s) }
                )
            )
        )
        .add_subtree(
            "Raw Material",
            menu::MenuTree::new()
            .subtree(
                "Grades",
                menu::MenuTree::new()
                .leaf(
                    "New Grade",
                    |s|{ create_grades(s) }
                )
            )
        );

    sign_in(&mut siv);

    siv.run();
}