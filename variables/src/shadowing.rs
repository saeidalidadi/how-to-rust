/**
 * we can shadow a variable in rust without any fear.
 * it means that you can declare and assing to it many times
 * in just one scope.
 */
fn main() {
    let shadow = 5;

    let shadow = 5 - shadow;

    let shadow = shadow * 3;

    println!("The value of x is: {}", shadow);
}