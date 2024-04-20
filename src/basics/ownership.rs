/*
creates a string with name some_str and move out the
value to the assigning variable
*/
pub fn gives_ownership() -> String {
    let some_str = String::from("yours");

    some_str
}

/*
takes the ownership of str and gives it back to the assigning variable
*/
pub fn takes_and_giveback(str: String) -> String {
    str
}

pub fn takes_print_and_giveback(str: String) -> String {
    let s_tmp = str;
    println!("print(str): {}", s_tmp);
    s_tmp
}
