pub fn data_types() {
    println!("Hello, world!");

    let _decimal = 98_222;
    let _hex	= 0xff;
    let _octal = 0o77;
    let _binary = 0b1111_0000;
    let _byte = b'A'; // (u8 only)
    let _bit_u64 = 2022u64; // suffix an integer literal with an integer type

    let array: [u32; 3] = [1, 2, 3];
    println!("{}", array[0]); // 1
    println!("{}", array[1]); // 2
    println!("{}", array[2]); // 3

    // An array is of a known length. Often, however, you would like to be able to write code that can handle arrays of any length. This is where slices come in. The slice type is written as [T] where T is the type. Notice that the length is not specified. 👉The slice is known as an unsized type or a dynamically sized type (DST). It is not possible to store an unsized type on the stack.

    let _a: &[u8] = &[1, 2, 3];

    // Generally, a &[T] denotes a reference to a slice. Normal references, like &u32 refer to the memory address of a u32 and are therefore the size of a single pointer. A reference to an unsized type requires more information.

    // Another prevalent unsized type is the str type. As mentioned before, the str type represents a UTF-8 encoded string of unknown size. 👉 By placing the str behind a reference, you can store their values on the stack:
    let _str: &str = "Hello World!";

    let tuple: (bool, u32, f64) = (true, 2, 3.0);
    println!("{}", tuple.0); // true
    println!("{}", tuple.1); // 2
    println!("{}", tuple.2); // 3.0

    // Tuple structs
    struct MyTuple(bool, u32, f64);
    let _tuple = MyTuple(false, 2, 3.0);

    // type alias
    type _MyTupleAlias = (bool, u32, f64);
    // ❗️ Notice that this is a type alias; you are not creating a new type. This is where tuple structs are different from aliased tuples.


    struct MyStruct {
        _should_do_groceries: bool,
        _birth_year: u32,
        _height_in_meters: f64,
    }

    let _my_struct = MyStruct {
        _should_do_groceries: true,
        _birth_year: 1992,
        _height_in_meters: 1.79,
    };


    enum CardinalDirection {
        _North,
        East,
        _South,
        _West,
    }

    let d = CardinalDirection::East;

    if let CardinalDirection::East = d {
        println!("We are going east!");
    } else {
        println!("We are not going east but in some other direction!");
    }


    enum Shape {
        Square {
            side: f64
        },
        Rectangle {
            width: f64,
            height: f64,
        },
        Circle {
            radius: f64,
        },
    }

    let shape_rect = Shape::Rectangle {
        width: 800.0,
        height: 60.0,
    };

    let shape_sq = Shape::Square { side: 70.0 };
    let shape_cir = Shape::Circle { radius: 10.0 };

    match shape_rect {
        Shape::Square { side } => {
            println!("A {}x{} square!", side, side);
        }
        Shape::Rectangle { width, height } => {
            println!("A {}x{} rectangle!", width, height);
        },
        Shape::Circle { radius } => {
            println!("A circle of radius {} and diameter {}!", radius, radius * 2.0);
        }
    }

    match shape_sq {
        Shape::Square { side } => {
            println!("A {}x{} square!", side, side);
        }
        Shape::Rectangle { width, height } => {
            println!("A {}x{} rectangle!", width, height);
        },
        Shape::Circle { radius } => {
            println!("A circle of radius {} and diameter {}!", radius, radius * 2.0);
        }
    }

    match shape_cir {
        Shape::Square { side } => {
            println!("A {}x{} square!", side, side);
        }
        Shape::Rectangle { width, height } => {
            println!("A {}x{} rectangle!", width, height);
        },
        Shape::Circle { radius } => {
            println!("A circle of radius {} and diameter {}!", radius, radius * 2.0);
        }
    }
}
