/**
 * Rust uses `for...in`, `while`, and `loop` for looping.
 * Also for conditions we should use.
 * `if statement {
 *  } else if statement {
 *  } else {
 *  }
 *  Rust is an Strongly Typed language so we can't use other types such as
 *      `number` and `string` for condition statements.
 *  ex:
 *      let number = 5;
 *      if number {}; // Mismatched type error
 */

// `for...in`
fn main() {
    println!("Hello, Loops!!!");
    let mut counter = 10;
    while counter > 0 {
        println!("counter is {}", counter);
        counter = counter - 1;
    }

    for i in 1..8 {
        
        // `if condition {}` control flow
        if i % 2 == 0 {
            println!("{} is and 'Even' number.", i)
        } else {
            println!("{} is an 'Odd' number.", i)
        }
    }

    // loop through elements of a collection
    let names = ["John", "Jack", "Saeid"];

    for name in names {
        println!("Name is: {}", name);
    }
}
