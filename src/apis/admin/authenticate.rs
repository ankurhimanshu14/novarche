pub mod authenticate {

    use mysql::*;
    use mysql::prelude::*;

    use crate::apis::admin::user_signup::user_signup::User;

    use bcrypt::{ hash, verify, DEFAULT_COST };

    pub fn get_user(u: String) -> Result<Vec<User>> {
        let query = format!("SELECT employee_id, username, hash, roles_name
        FROM user
        WHERE username = '{}';", u);

        let url = "mysql://root:@localhost:3306/mws_database".to_string();

        let pool = Pool::new(url)?;

        let mut conn = pool.get_conn()?;

        let mut v: Vec<User> = Vec::new();

        let if_exist = "SELECT COUNT(*)
            FROM information_schema.tables 
            WHERE table_schema = DATABASE()
            AND table_name = 'user';";

        let exists = conn.query_map(
            if_exist,
            |count: usize| {
                count
            }
        ).unwrap();

        let res = match &exists[0] {
            0 => vec![()],
            _ => {
                conn.query_map(
                    query,
                    |(employee_id, username, hash, role)| {
                        v.push(User{employee_id, username, hash, role})
                    }
                ).unwrap()
            }
        };

        Ok(v)
    }

    pub fn verify_user(u: User, p: String) -> Result<bool> {
        let res = match verify(u.hash, &p) {
            Ok(_) => true,
            Err(_) => false
        };

        Ok(res)
    }
}