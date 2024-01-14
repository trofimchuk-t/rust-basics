pub fn run() {
    let age = 18;

    if age >= 21 {
        //
    } else if age < 18 {
        //
    } else {
        //
    }

    // shorthand if
    let is_of_age = if age > 21 { "yes" } else { "no" };
    println!("{}", is_of_age);
}
