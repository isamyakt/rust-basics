
// Without Generics Types
// sequence of the type i32
pub struct SequenceI32 {
    pub first: i32,
    pub second: i32,
    pub third: i32,
}

// sequence of 3 String values
pub struct SequenceString {
    pub first: String,
    pub second: String,
    pub third: String,
}

impl SequenceI32 {
    pub fn new(first: i32, second: i32, third: i32) -> Self {
        Self { first, second, third }
    }
}

impl SequenceString {
    pub fn new(first: String, second: String, third: String) -> Self {
        Self { first, second, third }
    }
}


// With Generic Types
pub struct Sequence<T> {
    pub first: T,
    pub second: T,
    pub third: T,
}

// Read this as: for any type `T`, implement for `Sequence<T>` ...
impl<T> Sequence<T> {
    pub fn new(first: T, second: T, third: T) -> Self {
        Self { first, second, third }
    }


    // below all_same() fails to compile with an error because
    // binary operation `==` cannot be applied to type `T.`

    // pub fn all_same(&self) -> bool {
    //     self.first == self.second && self.second == self.third
    // }
}


// implementations for the types that support explicitly.
impl Sequence<i32> {
    pub fn all_same(&self) -> bool {
        self.first == self.second && self.second == self.third
    }
}

pub fn checking_all_same() {
    let s = Sequence { first: 1, second: 2, third: 3 };
    println!("{}", s.all_same());
}



// Generic Type Parameter Bounds
use std::cmp::PartialEq;

// For all types T implementing PartialEq, implement for Sequence3<T> ...
impl<T: PartialEq> Sequence<T> {
    pub fn all_same_types(&self) -> bool {
        self.first == self.second && self.second == self.third
    }
}

// OR

// We can also move the type bound to a `where` clause.
impl<T> Sequence<T> where T: PartialEq {
    pub fn all_same_type(&self) -> bool {
        self.first == self.second && self.second == self.third
    }
}



// Associated Types in Trait Bounds
use std::ops::Add;

impl<T> Sequence<T> where T: Copy + Add<Output = T> {
    pub fn sum(&self) -> T {
        self.first + self.second + self.third
    }
}



// Using Multiple Generic Type Parameters
pub struct MyStruct<A, B> {
    pub a: A,
    pub b: B,
}

pub enum MyEnum<A, B> {
    A(A),
    B(B),
}

pub fn multiple_generics() {
    let _s = MyStruct { a: 10, b: "Hello" };

    // We have to specify the type of the `MyEnum::A` variant here because Rust does not have
    // information to infer it.
    let _e = MyEnum::<i32, _>::B("Hello");
}



// Generic Functions
// Accept any type `T` implements `Display` meaning that they can be formatted as text.
pub fn say_hello<T: std::fmt::Display>(value: &T) {
    println!("Hello, {value}!");
}

pub fn main() {
    say_hello(&true); // Hello, true!
    say_hello(&String::from("World")); // Hello, World!
    say_hello(&17); // Hello, 17!
}