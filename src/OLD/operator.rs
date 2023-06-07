use std::collections::HashMap;
use lazy_static::lazy_static;

// creates a static variable that maps enum variants to functions. Useful for operator
macro_rules! variant_functions {
    ($enum_type:ty $(, $variant:ident => $function:ident)*) => {
        struct VariantFunctions {
            function_map: &'static HashMap<$enum_type, fn(&$enum_type)>,
        }

        impl VariantFunctions {
            fn new() -> VariantFunctions {
                let mut function_map = HashMap::new();
                $(function_map.insert(<$enum_type>::$variant, <$enum_type>::$function as fn(&$enum_type));)*

                VariantFunctions {
                    function_map: Box::leak(Box::new(function_map)),
                }
            }

            fn call_function(&self, enum_value: &$enum_type) {
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
