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
