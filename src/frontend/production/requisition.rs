pub mod requisition {

    use cursive::{
        Cursive,
        view::{ Nameable, Resizable },
        views::{  Dialog, EditView, ListView, TextView },
    };

    use crate::apis::production::requisition::requisition::Requisition;

    pub fn raise_requisition(s: &mut Cursive, p: usize, from: String, to: String) {
        s.pop_layer();
        s.add_layer(
            Dialog::new()
            .title("Raise Cutting Requisition")
            .padding_lrtb(1, 1, 1, 0)
            .content(
                ListView::new()
                .child("Part No", TextView::new(p.clone().to_string()))
                .child("Requested Qty (Nos)", EditView::new().with_name("req_qty").fixed_width(30))
                .child("Comments", EditView::new().with_name("comments").fixed_width(30).min_height(5))
            )
            .button(
                "Send Request",
                move |s| {

                    let p_no = p.clone();

                    let req_qty = s.call_on_name("req_qty", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();
                    
                    let comments = s.call_on_name("comments", |v: &mut EditView| {
                        v.get_content()
                    }).unwrap();

                    let comments = match comments.clone().len() {
                        0 => None,
                        _ => Some(comments.clone().to_string())
                    };

                    match Requisition::new(from.clone(), to.clone(), p_no, req_qty.to_string().parse::<usize>().unwrap(), comments).post() {
                        Ok(0) => s.add_layer(Dialog::info("Check request entries again")),
                        Ok(m) =>{
                            s.pop_layer();
                            s.add_layer(Dialog::text(format!("Requisition raised successfully. Insert ID: {}", m)).dismiss_button("Ok"))
                        },
                        Err(e) => s.add_layer(Dialog::text(format!("Error encountered: {}", e)).dismiss_button("Ok"))
                    }

                }
            )
            .dismiss_button("Ok")
        )
    }
}