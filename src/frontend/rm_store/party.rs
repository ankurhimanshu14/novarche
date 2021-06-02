pub mod party {

    use cursive::{
        Cursive,
        view::{ Nameable, Resizable },
        views::{ Dialog, EditView, ListView, TextArea },
    };

    use crate::apis::rm_store::party::party::Party;

    pub fn create_party(s: &mut Cursive) {
        s.add_layer(
            Dialog::new()
            .title("Create Party")
            .padding_lrtb(1,1,1,1)
            .content(
                ListView::new()
                .child("Party Code", EditView::new().with_name("party_code").fixed_width(30).min_height(2))
                .child("Party Name", EditView::new().with_name("party_name").fixed_width(30).min_height(2))
                .child("Party Address", TextArea::new().with_name("party_address").fixed_width(30).min_height(5))
            )
            .button(
                "Add",
                |s| {
                    let party_code = s.call_on_name("party_code", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let party_name = s.call_on_name("party_name", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let party_address = s.call_on_name("party_address", |v: &mut TextArea| {
                        v.get_content().to_string()
                    }).unwrap();

                    match Party::new(party_code.to_string(), party_name.to_string(), party_address.to_string()).post() {
                        Ok(_) =>{
                            s.pop_layer();
                            s.add_layer(Dialog::text("Party details added successfully").dismiss_button("Ok"))
                        },
                        Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                    }
                }
            )
            .dismiss_button("Cancel")
        )
    }
}