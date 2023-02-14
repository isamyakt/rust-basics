// Pass By Value
pub fn length_of_string(value: String) -> usize {
    value.len()
}

pub fn main_fn() {
    let s1 = String::from("Hey Rustaceans!");
    let len = length_of_string(s1);
    println!("The length is {len}.");
}

pub fn main_fn_error() {
    // moved value cannot be used, so thats the error
//     let s1 = String::from("Hey there!");
//     let len = length_of_string(s1);
//     println!("The length of {s1:?} is {len}.");

    // s1 to the length_of_string function is by duplicating the data first
    let s1 = String::from("Hey there!");
    let s2 = s1.clone();
    let len = length_of_string(s2);
    println!("The length of {s1:?} is {len}.");
}

// You do have to realize that cloning the String comes with a small
// performance cost. You need to allocate a new stringand copy
// the bytes over. Besides that, perhaps, more importantly,
// you can now do things that make the output incorrect
pub fn main_fn_incorrect_output() {
    let mut s1 = String::from("Hey there!");
    let s2 = s1.clone();
    s1.push_str(" How are you?");
    let len = length_of_string(s1);
    println!("The length of {s2:?} is {len}.");
}