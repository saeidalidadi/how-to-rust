/**
 * we can use the shadowing for one variable name with two different types
 */
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces is: {}", spaces);
}