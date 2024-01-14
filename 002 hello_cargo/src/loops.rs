pub fn run() {
    let mut counter = 0;

    // infinite loop:
    loop {
        counter += 1;
        println!("Loop {0}", counter);
        if counter > 20 {
            break;
        }
    }

    counter = 0;
    while counter <= 100 {
        if counter % 15 == 0 {
            println!("fizzbuzz");
        }

        counter += 1;
    }

    // for range
    for x in 1..100 {
        if x % 15 == 0 {
            println!("{} fizzbuzz", x);
        }
    }
}
