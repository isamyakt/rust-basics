macro_rules! create_vec {
    ( $( $item:expr ),* ) => {
        {
            let mut result = Vec::new();
            $(
                result.push($item);
            )*
            result
        }
    }
}

pub fn macros() {
    let items = create_vec!(1, 2, 3);
    println!("{items:?}");
}


// Custom Derive Macros

// #[derive(Default)]
struct MyType {
    name: String,
    items: Vec<i32>,
}

// use anyone: 
// #[derive(Default)] 
//        OR
// impl Default for MyType

impl Default for MyType {
    fn default() -> Self {
        Self {
            name: Default::default(),
            items: Default::default(),
        }
    }
}

pub fn my_type() {
    let v = MyType::default();
    assert!(v.name.is_empty());
    assert!(v.items.is_empty());
}


// Attribute-Like Macros
// The macro can transform the code it applies to into any other code.

// #[my_attr_macro]   // uncomment
fn _x() {}

// #[my_attr_macro]   // uncomment
const _Y: u32 = 1;

// #[my_attr_marco]   // uncomment
struct _Z;



// Function-Like Macros
// Function-like macros can generate any code from its input code

// my_fn_macro!(some stuff);  // uncomment