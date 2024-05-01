pub mod ownership;
pub mod ref_borrowing;
pub mod slices;
pub mod structs;
pub mod traits_lifetimes;

pub use crate::basics::ownership::*;
pub use crate::basics::ref_borrowing::*;
pub use crate::basics::slices::*;
pub use crate::basics::structs::*;
pub use crate::basics::traits_lifetimes::*;

pub fn show_basics() {
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

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("user2 with different email: {:?}", user2);

    let new_coordinate: Coordinate<f32> = Coordinate { x: 10.2, y: 12.9 };

    println!(
        "Distance from origin: {}",
        new_coordinate.distance_from_origin()
    );
    println!("X coordinate: {}", new_coordinate.get_x());

    /* ERROR: println!("user2 with different email: {:?}", user1);
    struct update syntax uses = assignment because we're moving
    some other properties like username that is a string, so we
    can no longer use user1 as whole. If we had given new values
    for username and email then we wouldn't have any problems
    because the other fields are boolean or integers so they're
    saved in the stack and therefore they're not being moved at all
    (they implement the Copy trait)
    */

    /*Generics and Traits */
    let num_list = vec![34, 50, 25, 100, 65];

    match find_largest(&num_list) {
        None => println!("the list is empty"),
        Some(res) => println!("the biggest element in num_list is: {}", res),
    }

    let t1 = Transaction {
        id: 1,
        object: String::from("credit card"),
        amount: 100.0,
        quantity: 1,
    };

    notify_element(&t1);
}
