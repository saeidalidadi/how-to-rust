/**
 * fn (arg1: type, arg2: type, ....) -> return_type {
 * }
 * 
 */
fn main() {
    println!("This is a function call");
    println!("Return from Function => {}", say_hello(6));
}

fn say_hello(num: i8) -> i8 {
    println!("Hello Functions! called with {} as argument", num);
    num + 2 // we skipe that ; for returning result 
}