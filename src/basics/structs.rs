pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

/*If a struct is mutable, then all its attributes are mutable*/
pub fn change_username(new: &String, user: &mut User) {
    user.username = String::from(new);
}

pub fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
