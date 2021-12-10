/**
 * fn (arg1: type, arg2: type, ....) -> return_type {
 * }
 * 
 */
fn main() {
    println!("This is a function call");
    println!("Return from Function => {}", say_hello(6));


    // Function's statement with expressions
    println!("Hi {}", with_expression(5));
}

fn say_hello(num: i8) -> i8 {
    println!("Hello Functions! called with {} as argument", num);
    num + 2 // we skipe that ; for returning result 
}


/***
 * Statements & Expressions in Rust
 * Statement: Statements are instructions that perform some action and do not return a value
 * Expression: Expressions evaluate to a resulting value
 */
fn with_expression(num: i8) -> i8 {
    let my_name = {
       let a = 10;
       a + num
    };

    my_name
}