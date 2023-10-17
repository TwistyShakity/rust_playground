use std::fmt::Debug;

#[derive(Default, Debug, Clone)]
struct SomeStruct {
    int_field: i32,
    bool_field: bool,
    string_field: String,
}


fn print_debug_format<T: Debug>(target: T) {
    println!("{:#?}", target)
}
fn main() {
    let mut some_instance = SomeStruct {
        string_field: "Hello".into(),
        ..Default::default()
    };

    print_debug_format(some_instance);
}
