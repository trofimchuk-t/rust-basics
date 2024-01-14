/* Primitives:
- integer: u8, i8 ... u128, i128
- float: f32, f64
- bool: true | false
- char: 'a'
- tuples
- arrays
 */

pub fn run() {
    // default integer type is i32
    let x = 1;
    println!("x = {}, i32 MAX = {}", x , std::i32::MAX);

    // default float type is f64
    let f = 2.5;

    let z: f64 = 24243244.42334;

    // Bolleans:
    let is_active = true;
    let is_greater = 10 > 5;

    // chars:
    let a1 = 'a';
    let emodge = '\u{1F600}';

    println!("{:?}", (x, f, z, is_active, is_greater, a1, emodge))
}