// casts an enum variant to its type when it is known.
macro_rules! cast {
    ($target: expr, $pat: path) => {
        {
            if let $pat(a) = $target { // #1
                a
            } else {
                panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
            }
        }
    };
}

// casts an enum variant to its type when it is known, returning an Option
macro_rules! trycast {
    ($target:expr, $pat:path) => {
        if let $pat(a) = $target {
            Some(a)
        } else {
            None
        }
    };
}



use std::collections::HashMap;
use lazy_static::lazy_static;

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

macro_rules! variant_functions {
    ($($variant:ident => $function:ident),*) => {
        struct VariantFunctions {
            function_map: &'static HashMap<MyEnum, fn(&MyEnum)>,
        }

        impl VariantFunctions {
            fn new() -> VariantFunctions {
                let mut function_map = HashMap::new();
                $(function_map.insert(MyEnum::$variant, MyEnum::$function as fn(&MyEnum));)*

                VariantFunctions {
                    function_map: Box::leak(Box::new(function_map)),
                }
            }

            fn call_function(&self, enum_value: &MyEnum) {
                if let Some(function) = self.function_map.get(enum_value) {
                    function(enum_value);
                }
            }
        }

        lazy_static! {
            static ref VARIANT_FUNCTIONS: VariantFunctions = VariantFunctions::new();
        }
    };
}

variant_functions!(
    Variant1 => function_for_variant1,
    Variant2 => function_for_variant2,
    Variant3 => function_for_variant3
);

fn main() {
    let enum_value = MyEnum::Variant2;

    VARIANT_FUNCTIONS.call_function(&enum_value);
}
