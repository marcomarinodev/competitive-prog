/*
Assume that we want to return the first word fo a string having
words separated by spaces. If we just return the integer index
of where the first word terminates we could have runtime problems
as for example the string could be cleared then the index would be
no longer valid.

Rust solves this problem you could have at runtime by using slices
when there is a immutable pointer to a slice of the string then
it applies the borrowing rule: you cannot have an immutable
reference and at the same time a mutable reference otherwise you
could encounter data race problems
*/
pub fn first_word(str: &String) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }

    &str[..]
}

// fn main() {
//     let mut s = String::from("hello world");

//     let first_word = first_word(&s);

//     // s.clear() returns error, because there is already a immutable reference

//     println!("The first word is: {}", first_word);
// }
