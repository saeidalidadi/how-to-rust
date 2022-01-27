/**
 * Reuslt is a generic enum that is so common for rust's error handling
 * ```
 * enum Resukt<T, E> {
 *   Ok(T),
 *   Err(E)
 * }
 * ```
 * It is the idiomatic way in which the language does error handling.
 */


 fn maight_fail(n: i16) -> Result<f32, String> {
    if n < 3 {
        Ok(3.2)
    } else {
        Err(String::from("N should be less than 3"))
    }
 }

fn main() -> Result<(), String> {
    let result = maight_fail(4);

    match result {
        Ok(t) => println!("{}", t),
        Err(e) => println!("Error: {}", e)
    };

    let result = maight_fail(2)?;
    println!("Result {}", result);
    Ok(())

}
