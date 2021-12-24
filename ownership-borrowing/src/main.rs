/**
 * Ownership: Is a set of rules that Rust uses for memory managemnet 
 *  that happen at compile time.
 *  one of alternatives for ownership that some programming languages such as Java
 *  do, is the Garbage Collection paradigm.
 * 
 * Ruls:
 *  1. Each value in Rust has a variable that’s called its owner.
 *  2. There can only be one owner at a time.
 *  3. When the owner goes out of scope, the value will be dropped.
 *  
 */

fn main() {
    println!("Hello, world!");
}
