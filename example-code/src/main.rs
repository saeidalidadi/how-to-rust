/**
 * Insertion Sort
 * Complexity => n^2
 */

fn main() {
    let mut list = [2, 5, 1, 10, 0];
    let len = list.len();
    for i in 1..len {
        let mut k = i;
        while k > 0 && list[k - 1] > list[k] {
            list.swap(k - 1, k);
            k -= 1;
        }
    }
    println!("list is -> {:?} ", list)
}
