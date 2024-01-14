pub fn run() {
    // immutable, fixed-length type 'str':
    let hello = "Hello!";
    println!("Length: {}", hello.len());

    // growable, mutable data structure places in the heap 'String':
    let mut hello2 = String::from("hello");
    hello2.push('!');
    hello2.push_str(" I'm there!");
    println!("{} | Length: {} | Capacity: {}", hello2, hello2.len(), hello2.capacity());

    // hello2.is_empty()
    // hello2.contains("World")

    for word in hello2.split_whitespace(){
        println!("{}", word);
    }

    println!("Replace: {}", hello2.replace("hello", "Hi"));
    println!("hello2: {}", hello2);

    // create String with capacity:
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('c');
    println!("s: {}, len: {}, cap: {}", s, s.len(), s.capacity());

    // assertions:
    assert_eq!(2, s.len());
}