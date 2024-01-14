pub fn run() {
    let name = "John";
    //let age = 34;
    // Variables are immutable by default
    //age = 35; // error!

    let mut age = 33;
    println!("Age: {}", age);
    age = 34;

    println!("My name is {} and I'm {} years old", name, age);

    // Define constant (need to use type explicitly)
    const  ID: i32 = 32000000;
    println!("Const ID is {}", ID);

    // Assign multiple vars:
    let (my_name, my_age) = ("John", 24);
    println!("My name is {}, I'm {} old", my_name, my_age)
}