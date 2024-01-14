// Vectors - resizible arrays

use std::mem;

pub fn run() {
    // set type and length of vector
    let mut numbers: Vec<i32> = vec![111, 222, 333];
    numbers.push(444);
    numbers.push(555);
    numbers.pop();

    println!("{:?}", numbers);

    let mut mutable_vector: [i32; 3] = [111, 222, 333];
    println!("{:?}", mutable_vector);
    mutable_vector[mutable_vector.len() - 1] = -444;
    println!("{:?}", mutable_vector);

    // get allocated number of bytes:
    println!("{}", mem::size_of_val(&mutable_vector));

    // get slice:
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop:
    for x in numbers.iter() {
        println!("Number: {}", x)
    }

    // loop and mutate:
    for x in numbers.iter_mut() {
        *x += 1
    }
    println!("After mutating: {:?}", numbers);

}
