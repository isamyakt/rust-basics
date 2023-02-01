pub fn referential_types() {
    // Immutable References
    struct Config {
        port: u16,
    }
    let config: Config = Config {
        port: 8080
    };
    // Create a reference.
    let config_reference: &Config = &config;
    // In some other part of the program, use the reference.
    println!("Using port {}.", config_reference.port);
    


    // Multiple Immutable References
    let val = 10;
    let r1 = &val;
    let r2 = &val;
    println!("{r1} should be the same as {r2}.");



    // Mutable References
    struct Configs {
        port: u16,
    }
    let mut configs: Configs = Configs {
        port: 8080
    };
    // Create a mutable reference.
    let config_reference: &mut Configs = &mut configs;
    // In some other part of the program, use the reference.
    config_reference.port = 4000;
    // Observe the original having been modified.
    println!("Using port {}.", configs.port);


    // Only 1 Mutable Reference
    let mut val = 10;
    let r1 = &mut val;  // 1 mutable reference to a value simultaneously
    // let r2 = &mut val;  // If you use more then 1 mutable reference then you wil get error
    // Assign 5 to the value referenced by r1.
    *r1 = 5;
    


    // Deferencing
    let val: i32 = 10;
    let r1: &i32 = &val;
    // This creates a copy of the value 10.
    let val2: i32 = *r1;
    println!("deferencing val2 : {}", val2);



    // Copy
    let val: String = "Hello!".to_string();
    let r1: &String = &val;
    // Compilation error!
    // let val2: String = *r1;
    println!("deferencing val2 : {}", r1);
    
}