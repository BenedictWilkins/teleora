use std::collections::HashMap;
use lazy_static::lazy_static;

// Define your enum
#[derive(Debug, Eq, PartialEq, Hash)]
enum MyEnum {
    Variant1,
    Variant2,
    Variant3,
}

impl MyEnum {
    fn function_for_variant1(&self) {
        println!("Function called for Variant1");
    }

    fn function_for_variant2(&self) {
        println!("Function called for Variant2");
    }

    fn function_for_variant3(&self) {
        println!("Function called for Variant3");
    }
}

variant_functions!(
    MyEnum,
    Variant1 => function_for_variant1,
    Variant2 => function_for_variant2,
    Variant3 => function_for_variant3
);

fn main() {
    let enum_value = MyEnum::Variant2;

    VARIANT_FUNCTIONS.call_function(&enum_value);
}