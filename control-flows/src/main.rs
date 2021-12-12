/**
 * Rust uses `for...in`, `while`, and `loop` for looping.
 * Also for conditions we should use 
 * `if statement {
 *  } else if statement {
 *  } else {
 *  }
 * 
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
        if i % 2 == 0 {
            println!("{} is and 'Even' number.", i)
        } else {
            println!("{} is an 'Odd' number.", i)
        }
    }
}
