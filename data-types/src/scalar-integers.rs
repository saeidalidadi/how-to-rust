/**
 * integers:
 *  signed  unsined  range( -(2^(n-1))..2^(n-1) )
 *     i8     u8     -128..127
 *     i16    u16    
 *     i32    u32
 *     i64    u64
 *     i128   u128
 *     isize  usize
 */

fn main() {
    let num_1 = 1;

    let num_1: u32 = 55;

    // This cause an error
    let unsined_num: i8 = -129;

    let unsined_num: i16 = -i16::pow(2, 15) - 2;

}
