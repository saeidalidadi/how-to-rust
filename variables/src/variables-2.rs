/**
 * Mutable variables are not allowed to change their types
 */

fn main() {
    let mut age = "35";

    // this will cause a mismatched types error
    age = age.len();

    println!("I am {} years old", age);
}