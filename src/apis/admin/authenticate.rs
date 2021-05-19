pub mod authenticate {

    use mysql::*;
    use mysql::prelude::*;

    use crate::apis::admin::user_signup::user_signup::User;

    use bcrypt::{ hash, verify, DEFAULT_COST };

    pub fn verify_user(u: String, p: String) -> Result<bool> {
        let query = format!("SELECT employee_id, username, hash, roles_name
        FROM user
        WHERE username = '{}';", u);

        let url = "mysql://root:@localhost:3306/mws_database".to_string();

        let pool = Pool::new(url)?;

        let mut conn = pool.get_conn()?;

        let result = conn.query_map(
            query,
            |(employee_id, username, hash, role)| {
                let user = User {
                    employee_id,
                    username,
                    hash,
                    role
                };

                match verify(user.hash, &p) {
                    Ok(_) => true,
                    Err(_) => false
                }
            }
        )?;

        Ok(result[0])
    }
}