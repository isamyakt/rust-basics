pub fn var_mut() {
    // Default Immutablity
    let x;
    x = 7;
    println!("default x: {}", x);



    // Type Ascription
    let x: i16 = 7;
    println!("ascription x: {}", x);



    // Mutability
    let mut _y = 4;
    _y = 7;
    println!("mutable y: {}", _y);



    // Scoping
    let x = 7;
    println!("{}", x); // 7
    // Create a new scope
    {
        let y = 17;
        // We can use x here
        println!("{}", x); // 7
        println!("{}", y); // 17
    }
    println!("{}", x); // 7
    // println!("{}", y); // won't compile because y is "not in scope"




    // Shadowing
    let x = 7;
    println!("Shadowing x: {}", x);
    let x =  17;
    println!("Shadowing x: {}", x);

    // Nested Scope Shadowing
    let x = 2;
    {
        let x = 3;
        println!("nested x: {}", x);
    }
    println!("{}", x); // 2




    // Patterns
    let (x, y) = (2, 3);  // Destructuring a Tuple
    println!("patterns x: {}, y: {}", x, y);


    // Define the `Person` type.
    struct Person {    
        name: &'static str,
        age: u32,
        likes_brownies: bool,
    }
    // Create a `Person` from its parts.
    let p = Person {
        name: "Mick",
        age: 30,
        likes_brownies: true,
    };
    // Deconstruct the `Person` back into its parts,
    // omitting fields other than `name` and `age`.
    
    // Destructure Structs
    let Person { name: x_name, age: x_age, likes_brownies: tr_or_fal } = p;

    assert_eq!("Mick", x_name);
    assert_eq!(30, x_age);
    assert_eq!(true, tr_or_fal);

}