#[derive(Debug)]
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

pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coordinate<T> {
    pub fn get_x(&self) -> &T {
        &self.x
    }

    pub fn get_y(&self) -> &T {
        &self.y
    }
}

impl Coordinate<f32> {
    pub fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
