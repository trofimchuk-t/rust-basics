// Max 12 elements

pub fn run() {
    let person = ("John", "Doe", 34);

    println!("{:?}", person);
    println!("Full name: {} {}, age: {}", person.0, person.1, person.2)
}