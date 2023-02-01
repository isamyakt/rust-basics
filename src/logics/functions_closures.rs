pub fn functions_closures() {
    // u32 value function
    fn u32_add(a: u32, b: u32) -> u32 {
        return a + b;
    }
    println!("a + b = {}", u32_add(7, 17));

    // OR 

    fn u32_adds(a: u32, b: u32) -> u32 {
        a + b
    }
    println!("a + b = {}", u32_adds(7, 17));




    // functions inside of another function
    fn f(n: u32) -> u32 {
        //The function g can not be accessed outside of f
        fn g(n: u32) -> u32 {  
            n + 1
        }
    
        g(n * 2)
    }
    println!("{}", f(3));




    // Associated Functions
    struct X(&'static str);
    // An implementation block for the type `X`.
    impl X {
        // An associated function.
        fn associated_fn() -> &'static str {
            "I am always the same!"
        }
        // A method.
        fn method(self: &Self) -> &'static str {
            self.0
        }
    }
    // Call a function associated to the type `X`.
    println!("{}", X::associated_fn());
    // Create an instance of X and call a method on the instance.
    let instance = X("My value depends on an instance of `X`!");
    println!("{}", instance.method());





    // Closures
    let c = |x| {
        x * 2
    };
    println!("{}", c(6));

    // OR

    let c = |x| x * 2;
    println!("{}", c(6));



    // Closures with environment
    let mut n = 0;
    let mut c = |x| {
        n += 1;
        x + n - 1
    };
    println!("{}", c(2));
    println!("{}", c(2));
    println!("{}", c(2));
    


    // Closures with iterating over collections of values
    let a = [1, 2, 3];
    let n: i32 = a.iter().map(|x| x * 2).sum();
    println!("Sum of {:?} after doubling: {}", a, n);
}