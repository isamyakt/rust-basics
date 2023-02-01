pub fn statements_expressions() {
    let brownies_eaten = 0;

    if brownies_eaten == 0 {
        println!("I had no brownies today.");
    } else {
        println!("I had at least one brownie today.");
    }

    // OR

    let quantifier;
    if brownies_eaten == 0 {
        quantifier = "no brownies";
    } else {
        quantifier = "at least one brownie";
    }
    println!("I had {quantifier} today.");

    // OR

    let quantifier = if brownies_eaten == 0 {
        "no brownies"
    } else {
        "at least one brownie"
    };
    println!("I had {quantifier} today.");

    // OR

    let quantifier = match brownies_eaten {
        0 => "no browines",
        1 => "at least one brownie",
        _ => "multiple brownies"
    };
    println!("I had {quantifier} today.");



    // Scopes
    let x;
    {
        println!("`x` has no value yet!");
        x = 3; // notice the final semicolon
    }
    println!("x : {}", x);

    // OR

    let x: u32 = {
        println!("`x` has no value yet!");
        3 // notice the lack of a semicolon, indicating an expression.
    }; // notice the final semicolon, the let statement needs to end somewhere!
    println!("x_u32: {}", x);



    // Loop
    let mut i = 0;
    let x = loop {
        if i > 7 {
            break i;
        }
        i += i*2 + 1;
    };
    println!("{x}");
}