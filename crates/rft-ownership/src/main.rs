mod ownership;
mod borrowing;

fn main() {
    let s_for_print = String::from("Hello, world!"); // memory allocation, s_for_print is the owner of the string

    handle_print(s_for_print); // ownership is transferred to the function

    // println!("{}", s_for_print); // error: we cannot use `s_for_print` after the function call because the ownership has been transferred

    let mut s_for_add_suffix = String::from("Hello"); // memory allocation, s_for_add_suffix is the owner of the string

    add_suffix(&mut s_for_add_suffix); // ownership is not transferred, just borrowed and it can be modified

    println!("{}", s_for_add_suffix); // this is valid because s_for_add_suffix is still the owner of the string
}

/**
 * Takes ownership of the string and prints it.
 */
fn handle_print(s: String) {
    println!("{}", s);
}

/**
 * This function use mutable references to modify the string.
 */
fn add_suffix(s: &mut String) {
    s.push_str(" (suffix)");
}