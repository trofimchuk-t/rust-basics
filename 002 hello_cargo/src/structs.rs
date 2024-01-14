// traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct
struct ColorT(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // construnct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut color: Color = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    color.blue = 128;

    println!("Color {} {} {}", color.red, color.green, color.blue);

    let mut colort = ColorT(255, 128, 128);
    colort.0 = 0;
    println!("ColorT {} {} {}", colort.0, colort.1, colort.2);

    let mut p = Person::new("John", "Wick");
    println!("Person {} {}", p.first_name, p.last_name);
    p.set_last_name("Doe");
    println!("Person {}", p.full_name());
    println!("Person tuple {:?}", p.to_tuple());
}
