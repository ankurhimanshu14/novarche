pub mod requisition {

    use mysql::*;
    use mysql::prelude::*;

    use crate::apis::utils::gen_uuid::gen_uuid::generate_uuid;

    #[derive(Debug, Clone)]
    pub struct Requisition {
        pub requisition_id: String,
        pub request_from: String,
        pub request_to: String,
        pub part_no: usize,
        pub requested_qty: usize,
        pub comments: Option<String>,
        pub reply: Option<String>
    }

    impl Requisition {
        pub fn new(
            request_from: String,
            request_to: String,
            part_no: usize,
            requested_qty: usize,
            comments: Option<String>
        ) -> Self {
            Requisition {
                requisition_id: generate_uuid(),
                request_from,
                request_to,
                part_no,
                requested_qty,
                comments,
                reply: None
            }
        }

        pub fn post(&self) -> Result<u64> {

            let table = "CREATE TABLE IF NOT EXISTS requisition (
                id                  INT                 NOT NULL                    PRIMARY KEY                 AUTO_INCREMENT,
                requisition_id      VARCHAR(100)        NOT NULL                    UNIQUE,
                request_from        VARCHAR(20)          NOT NULL,
                request_to          VARCHAR(20)          NOT NULL,
                part_no             INT                 NOT NULL,
                requested_qty       INT                 NOT NULL,
                comments            TEXT,
                reply               TEXT,
                created_at          DATETIME            NOT NULL                    DEFAULT                     CURRENT_TIMESTAMP,
                modified_at         DATETIME                                        ON UPDATE                   CURRENT_TIMESTAMP
            )ENGINE = InnoDB;";

            let url: &str = "mysql://root:@localhost:3306/mws_database";

            let pool: Pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            conn.query_drop(table)?;

            let insert = "INSERT INTO requisition(
                requisition_id,
                request_from,
                request_to,
                part_no,
                requested_qty,
                comments,
                reply
            ) VALUES (
                :requisition_id,
                :request_from,
                :request_to,
                :part_no,
                :requested_qty,
                :comments,
                :reply
            );";

            conn.exec_drop(
                insert,
                params! {
                    "requisition_id" => self.requisition_id.clone(),
                    "request_from" => self.request_from.clone(),
                    "request_to" => self.request_to.clone(),
                    "part_no" => self.part_no.clone(),
                    "requested_qty" => self.requested_qty.clone(),
                    "comments" => self.comments.clone(),
                    "reply" => self.reply.clone()
                }
            )?;

            Ok(conn.last_insert_id())
        }

        pub fn get_requisition(dept: String) -> Result<Self> {
            let select = "SELECT requisition_id, request_from, request_to, part_no, request_qty, comments, reply FROM requisition WHERE request_to = '{}' ORDER BY created_at;";

            let url: &str = "mysql://root:@localhost:3306/mws_database";

            let pool: Pool = Pool::new(url)?;

            let mut conn = pool.get_conn()?;

            let req = conn.query_map(
                select,
                |(requisition_id, request_from, request_to, part_no, request_qty, comments, reply)| {
                    Requisition{
                        requisition_id, request_from, request_to, part_no, request_qty, comments, reply
                    }
                }
            )?;

            Ok(req)
        }
    }
}