pub fn control_flow() {
    // if else Statement
    let value = 10;
    if value > -10 && value < 10 {
        println!("Single digit!");
    } else {
        println!("Multiple digits!");
    }



    // Multiple Conditions in Sequence
    let value = 10;
    if value == 0 {
        println!("Zero!");
    } else if value > -10 && value < 10 {
        println!("Single digit!");
    } else {
        println!("Multiple digits!");
    }



    // loops indefinitely
    loop {
        println!("I can't stop!");
        break; // if break is un-commented then to the loop doesn't stop
    }

    let mut i = 10;
    loop {
        if i == 0 {
            break;
        }
        println!("{i}...");
        i -= 1;
    }
    println!("Launch!");




    // while loop
    let mut x = 10;
    while x != 0 {
        println!("{x}...");
        x -= 1;
    }
    println!("Launch!");



    // for loop 
    for y in (1..=10).rev() {
        println!("{y}...");
    }
    println!("Launch!");

    // skip even numbers
    for i in (1..=10).rev() {
        if i % 2 == 0 {
            continue;
        }
        println!("{i}...");
    }
    println!("Launch!");
}