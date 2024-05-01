use crate::problems::replace_elements;

pub mod basics;
pub mod problems;
pub mod hands_on;

fn main() {
    let arr = vec![17,18,5,4,6,1];

    println!("result: {:?}", replace_elements(arr));
}
