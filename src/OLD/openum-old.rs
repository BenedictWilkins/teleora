/// Generates an enum whoses variants contain a method that performs a given binary operation. 
/// The operation must be defined by a trait with th
use crate::operator;


macro_rules! generate_binary_operator {
    ($enum_name:ident, $($variant:ident($operator:path, $operator_fn:ident)),*) => {
        $(
            #[derive(Debug, PartialEq, Eq)]
            pub struct $variant;
        )*
        
        $(impl $variant {
            fn evaluate<T, S>(&self, a: T, b: S) -> T::Output where T: $operator {
                a.$operator_fn(b)
            }
        })*
        
        #[derive(Debug, PartialEq, Eq)]
        pub enum $enum_name {
            $(
                $variant($variant),
            )*
        }
    };
}



generate_binary_operator!(Operator,
    Add(operator::Add<S>, add)
);

fn main() {
    use crate::operator::Add;
    let x : i32 = 1;
    x.add(1.0);
    x.add(1);



    //let add = Operator::Add(Add);
    println!("{:?}",Add.evaluate(1,2));
   
   // Add.evaluate(0.1, 1);
    //let another_enum_value = MyEnum::GreaterThan();
    //another_enum_value.evaluate();
}