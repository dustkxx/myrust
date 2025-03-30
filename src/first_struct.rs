
pub struct  User {
    pub name: String,
    pub email: String,
    pub sign_in_count: u64,
    pub age: u32,
    pub active: bool,
}


pub fn build_user(name: String, email: String) -> User {
    User {
        name,
        email,
        sign_in_count: 0,
        age: 0,
        active: false,
    }
}

pub fn get_user_name(user: &User) -> &String {
    &user.name
}
pub fn get_user_email(user: &User) -> &String {
    &user.email
}
