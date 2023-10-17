#[derive(Default, Debug)]
struct SomeStruct {
    int_field: i32,
    bool_field: bool,
    string_field: String,
}

fn main() {
    let mut some_instance = SomeStruct {
        string_field: "Hello".into(),
        ..Default::default()
    };

    {
        let move_string = some_instance.string_field;
    }
    some_instance.string_field = "Bye".into();

    println!("{:#?}", some_instance);
}
