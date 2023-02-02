use std::fmt::{Formatter, Error};


pub trait Displays {  // New Display
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}

// implementation of Display for the type bool
impl Displays for bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Display::fmt(if *self { "true" } else { "false" }, f)
    }
}

// The resulting &'static str is then displayed using it's implementation of Display
impl Displays for str {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.pad(self) // This implementation defers to the formatter's pad 
                    // function which handles the string padding and alignment.
    }
}

pub fn traits() {
    let value = "Hello";
    println!("regular: {}", value);
    println!("padded : {:_>8}", value);
}





// Bounds
// example 1
use std::fmt::Display;

pub struct MyStruct<A, B> {
    pub a: A,
    pub b: B,
}


impl<A: Display, B: Display> MyStruct<A, B> {
    pub fn print(&self) {
        println!("a: {}, b: {}", self.a, self.b);
    }
}

pub fn bound_trait() {
    let my_struct = MyStruct { a: 32, b: 10.0};
    println!("my struct: {} & {}", my_struct.a, my_struct.b);
}

// example 2
struct Person {
    name: String,
    age: u32,
}

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person {{ name: {}, age: {} }}", self.name, self.age)
    }
}


pub fn person_name_age() {
    let person = Person { name: "John".to_string(), age: 30 };
    println!("{}", person);
}




// Derive
// They can be derived based on the type definition.

// For example, let's take the Debug trait
// https://doc.rust-lang.org/std/fmt/trait.Debug.html

pub fn derive_debug() {
    // You can use it with the ? formatting option
    println!("{:?}", "Hello");
    println!("{:?}", vec!["Hello", "World"]);
}

// we can derive a Debug implementation with a procedural macro 
// defined by the standard library.
#[derive(Debug)]
pub struct Personx {
    pub name: String,
    pub age: i32,
}

pub fn procedural_macro_dervie() {
    let mick = Personx {
        name: "Mick".to_string(),
        age: 30
    };

    println!("{:?}", mick);
}