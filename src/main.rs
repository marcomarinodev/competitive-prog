pub mod basics;

use crate::basics::*;

fn main() {
    /*Ownership */
    let s1 = gives_ownership();
    let s2 = String::from("hello world");
    let mut s3 = takes_and_giveback(s2);
    s3 = takes_print_and_giveback(s3);
    println!("s3: {}", s3);
    println!("s1: {}", s1);

    /*Ref and Borrowing */
    let mut str = String::from("from string value");

    ref_change(&mut str);

    let str_len = ref_calculate_len(&str);

    println!("str_len: {}", str_len);

    /*Slices */
    // s3 is the one with the ownership, not s2
    let first_s2_word = first_word(&s3);

    println!("The first word of s2 is: {}", first_s2_word);

    /*Structs*/

    let mut user1 = build_user(
        String::from("someusername123"),
        String::from("someone@example.com"),
    );

    println!("old username: {}", user1.username);

    let new_username = String::from("new_username");
    change_username(&new_username, &mut user1);

    println!("new username: {}", user1.username);
}
