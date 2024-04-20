/*Here I am forced to move the str ownership to this function
and I am forced to give it back, so the return value will be
complicated (a pair basically)*/
pub fn own_calculate_len(str: String) -> (String, usize) {
    let len = str.len();
    (str, len)
}

/*So what I can do instead is to use the reference: in this way
the function does not take ownership of the string, but the function
borrows the value instead. But this makes the value immutable as
you cannot modify something that you do not own, but that you
borrow from someone else*/
pub fn ref_calculate_len(str_ref: &String) -> usize {
    str_ref.len()
}

/*You can actually modify something that you borrow by using
&mut reference, but remember that you can have 1 &mut at the time
referring to a specific variable. This restriction is used to avoid
data races.

You have this restriction even if there is a reference already and you
declare a mutable reference to the same variable then.

That's because users of immutable reference do not expect the value
to suddendly change due to an external mutable reference!
*/
pub fn ref_change(str_mut_ref: &mut String) {
    str_mut_ref.push_str(" !!!");

    /*
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    ==> NO PROBLEM HERE as the scope of r1 and r2 ends after the first
        println!(...)
    */
}
