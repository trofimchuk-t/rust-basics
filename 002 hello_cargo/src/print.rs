pub fn run() {
    // print to console
    println!("Print from print.rs file");

    // Positional formatting arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "John", "London", "code"
    );

    // Named formatting parameters:
    println!("{name} likes play {game}", name = "John", game = "soccer");

    // Placeholder traits:
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait:
    println!("{:?}", (12, true, "Hi!"));
}