pub mod forging {

    use chrono::NaiveDate;
    use mysql::*;
    use mysql::prelude::*;
    use uuid::Uuid;

    #[derive(Debug, Clone)]
    pub struct Forging {
        pub forging_id: String,
        pub planned_date: NaiveDate,
        pub machine: String,
        pub part_code: String,
        pub planned_qty: usize
    }

    impl Forging {
        pub fn new(
            pub planned_date: NaiveDate,
            pub machine: String,
            pub part_code: String,
            pub planned_qty: usize
        ) -> Self {
            Forging {
                cutting_id: Uuid::new_v4().to_string(),
                planned_date,
                machine,
                part_code,
                planned_qty
            }
        }
    }
}