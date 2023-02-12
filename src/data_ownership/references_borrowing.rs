// Pass By Value
pub fn length_of_string(value: String) -> usize {
    value.len()
}

pub fn main_fn() {
    let s1 = String::from("Hey Rustaceans!");
    let len = length_of_string(s1);
    println!("The length is {len}.");
}