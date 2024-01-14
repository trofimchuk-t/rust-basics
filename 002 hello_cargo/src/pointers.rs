pub fn run() {
    // primitive array (makes a copy)
    let mut arr1 = [1, 2, 3];
    let mut arr2 = arr1;

    arr1[0] = 999;
    arr2[0] = 55;
    arr1[2] = 0;

    println!("{:?}", (arr1, arr2));

    // with non-primitives, if you assign another variable to a piece of data, the first variable will not hold the data.
    // need to use a reference '&' to point to data

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    //vec1.push(4);

    println!("{:?}", (&vec1, vec2));

}