// array is fixed length list of values with same type

use std::mem;

pub fn run() {
    // set type and length of array
    let numbers: [i32; 3] = [111, 222, 333];

    println!("{:?}", numbers);

    let mut mutable_array: [i32; 3] = [111, 222, 333];
    println!("{:?}", mutable_array);
    mutable_array[mutable_array.len() - 1] = -444;
    println!("{:?}", mutable_array);

    // get allocated number of bytes:
    println!("{}", mem::size_of_val(&mutable_array));

    // get slice:
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}
