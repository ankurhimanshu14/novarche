mod apis;
mod frontend;

use cursive::{
    Cursive,
    CursiveExt,
    menu,
    views::{ Menubar },
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
        parts::parts:: {
            create_parts,
            get_part_list
        }
    },
    raw_material::{
        grades::grades::create_grades,
        steel::steel::{ create_steels, get_steel_list }
    },
    rm_store::{
        gate_entry::gate_entry::{ create_grn, assign_parts, get_gate_entry_list, get_approved_list },
        party::party::create_party
    },
    production::{
        cutting::cutting::{ plan,  update_cutting_status },
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
                .leaf(
                    "Part List",
                    |s|{ get_part_list(s) }
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
            .subtree(
                "Steels",
                menu::MenuTree::new()
                .leaf(
                    "New steel",
                    |s|{ create_steels(s) }
                )
                .leaf(
                    "Steel List",
                    |s| { get_steel_list(s) }
                )
            )
        )
        .add_subtree(
            "RM Store",
            menu::MenuTree::new()
            .subtree(
                "Party Details",
                menu::MenuTree::new()
                .leaf(
                    "New Party",
                    |s|{ create_party(s) }
                )
            )
            .subtree(
                "Gate Entry",
                menu::MenuTree::new()
                .leaf(
                    "New GRN",
                    |s|{ create_grn(s) }
                )
                .leaf(
                    "GRN List",
                    |s| { get_gate_entry_list(s) }
                )
                .leaf(
                    "Assign Approved Parts",
                    |s| { assign_parts(s) }
                )
                .leaf(
                    "Approved Parts List",
                    |s| { get_approved_list(s) }
                )
            )
        )
        .add_subtree(
            "Production",
            menu::MenuTree::new()
            .subtree(
                "Cutting",
                menu::MenuTree::new()
                .leaf(
                    "New Plan",
                    |s|{ plan(s) }
                )
                .leaf(
                    "Update Cutting Status",
                    |s|{ update_cutting_status(s) }
                )
            )
        );

    sign_in(&mut siv);

    siv.run();
}