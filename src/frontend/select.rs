pub mod select {
    use cursive:: {
        views::SelectView,
        align::HAlign,
    };

    use mysql::*;
    use mysql::prelude::*;

    pub fn select_view() -> Result<()> {
        let url = "mysql://root:@localhost:3306/mws_database".to_string();

        let pool = Pool::new(url)?;

        let mut conn = pool.get_conn()?;
        
        let query = "SELECT activity FROM authorities;";

        let mut v: Vec<String> = Vec::new();
        
        conn.query_map(
            query,
            |activity: String| {
                v.push(activity.to_string())
            }
        ).unwrap();

        let mut select = SelectView::new()
            .popup()
            .h_align(HAlign::Center)
            .autojump();

        select.add_all_str(v);

        Ok(())
    }
}