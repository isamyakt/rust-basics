pub fn main() {
    // Print the stack-size of a String.
    println!("The size of a `String` is {}", std::mem::size_of::<String>());
    // Create a String with a capacity of 4.
    let mut hello = String::with_capacity(4);
    // Print how the String is represented on the stack.
    print_string_stack_data(&hello);
    // Append the text "Hello!" to the (currently empty) String.
    hello.push_str("Hello!");
    // The capacity and length should have changed, and maybe the pointer.
    print_string_stack_data(&hello);
}
// Learning about unsafe Rust is out of scope so ignore this function.
fn print_string_stack_data(value: &String) {
    let ptr = value as *const _ as *const usize;
    println!("pointer  {0:16} 0x{0:016X}", unsafe { *ptr });
    println!("capacity {0:16} 0x{0:016X}", unsafe { *ptr.offset(1) });
    println!("length   {0:16} 0x{0:016X}", unsafe { *ptr.offset(2) });
}



// Borrowing 
pub fn borrowing() {
    let owner = 17; 
    let borrow = &owner;

    println!("ownership: {}", owner);
    println!("borrowing: {}", borrow);
}


// Copy and Clone
pub fn copy_clone() {
    let a = String::from("a");
    let b = a;
    // String types, which does not implement the Copy trait
    // println!("a = {a}");  // error because 'a' moves its value to 'b'.
    println!("a = {b}");


    let a = 10;
    let b = a;
    // Integer types, which allow to implement the Copy trait
    println!("a = {a}");
    println!("a = {b}");


    // Сopy Implemention with clone
    let a = String::from("a");
    let b = a.clone(); // Explicitly create a duplicate.
    println!("a = {a}");
    println!("a = {b}");
}