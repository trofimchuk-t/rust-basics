use std::str;

pub fn run() {
    greeting("Hello", "Ivan");

    println!("{}", add(10, 32));

    // closure (lambda? local func?):
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("{0}", add_nums(3, 3))
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
