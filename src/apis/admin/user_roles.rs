pub mod user_roles {
    pub struct Roles {
        pub role_id: i32,
        pub role_desc: String
    }

    pub struct UserRoles {
        pub role_id: i32,
        pub user_account_id: String
    }
}