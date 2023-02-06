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



#[derive(Default)]
struct MyType {
    name: String,
    items: Vec<i32>,
}

pub fn my_type() {
    let v = MyType::default();
    assert!(v.name.is_empty());
    assert!(v.items.is_empty());
}