/** 
 * We can shadow mutable variables and vise versa.
 * 
 */
fn main() {
    let mut age = 44;
    let age = "44";
    println!("She is {} years old!!", age);

    let my_name = "John";
    let mut my_name = "Smith";

    println!("My name is: {}", my_name);
}