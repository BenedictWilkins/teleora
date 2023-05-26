use pest::pratt_parser::{Assoc::*, Op, PrattParser};
use pest::iterators::{Pair, Pairs};
use crate::Rule as AstRule;

pub static DEBUG_DEFAULT : &str = "debug_default";

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

// pratt parser to handle operator prescendence
lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<AstRule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use AstRule::*;
        // Precedence is defined lowest to highest
        PrattParser::<AstRule>::new()
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
            .op(Op::prefix(minus))
    };
}

// implemented by each ast struct to pair from PEG parser
pub trait Parse {
    fn parse(pair : Pair<AstRule>) -> Self;
}

#[derive(Debug)]
pub struct Goal {
    pub head : Method,
    pub body : Vec<Rule>,
}
impl Default for Goal { 
    fn default() -> Self { return Goal { head : Method::default(), body : Vec::new() } }
}
impl Parse for Goal {
    fn parse(pair : Pair<AstRule>) -> Self {
        let mut pairs = pair.into_inner();
        // TODO here we should convert head into normal form (convert into goal conditions for later processing)
        let head : Method = Method::parse(pairs.next().unwrap()); // head can only take the form of a method 
        let body : Vec<Rule> = pairs.map(|p| Rule::parse(p)).collect(); 
        return Goal { head : head, body : body};
    }
}


#[derive(Debug)]
pub struct Rule { 
    pub conditions : Vec<Condition>,
    pub actions : Vec<Action>,
}
impl Default for Rule { 
    fn default() -> Self { return Rule { conditions : Vec::new(), actions : Vec::new() }; }
}
impl Parse for Rule {
    fn parse(pair : Pair<AstRule>) -> Self {
        let mut pairs = pair.into_inner(); // rules contain conditions -> actions
        let conditions : Vec<Condition> = pairs.next().unwrap().into_inner().map(|p| Condition::parse(p)).collect();
        let actions : Vec<Action> = pairs.next().unwrap().into_inner().map(|p| Action::parse(p)).collect();
        return Rule { conditions : conditions, actions : actions};
    }
}



#[derive(Debug)]
pub struct Condition(Statement);

impl Default for Condition { 
    fn default() -> Self { return Condition(Statement::Empty); }
}

impl Parse for Condition {
    fn parse(pair : Pair<AstRule>) -> Self {
        return Condition(Statement::parse(pair));
    }
}

#[derive(Debug)]
pub struct Action(Statement);

impl Default for Action {
    fn default() -> Self { return Action(Statement::Empty); }
}
impl Parse for Action {
    fn parse(pair : Pair<AstRule>) -> Self {
        match pair.as_rule() {
            AstRule::head => Action(Statement::Method(Method::parse(pair))),
            AstRule::atom => Action(Statement::Atom(Atom::parse(pair))),
            AstRule::variable => Action(Statement::Variable(Variable::parse(pair))),
            _ => unreachable!("Expected `action` or `subgoal` but received {pair}"),
        }
    }
}

#[derive(Debug)]
pub struct Method {
    pub name : Atom,
    pub arguments : Sequence
}
impl Default for Method {
    fn default() -> Self { return Method { name : Atom::default(), arguments : Sequence::default() }; }
}
impl Parse for Method {
    fn parse(pair : Pair<AstRule>) -> Self {
        let mut pairs = pair.into_inner();
        let name : Atom = cast!(Statement::parse(pairs.next().unwrap()), Statement::Atom);
        let arguments : Sequence = Sequence(pairs.map(|p| Statement::parse(p)).collect());
        return Method { name : name, arguments : arguments};
    }
}

#[derive(Debug)]
pub struct Variable {
    pub name : String,
}
impl Parse for Variable {
    fn parse(pair : Pair<AstRule>) -> Self {
        return Variable { name : pair.as_str().to_string() } ;
    }
}

#[derive(Debug)]
pub struct Atom(String);
impl Parse for Atom {
    fn parse(pair : Pair<AstRule>) -> Self {
        return Atom(pair.as_str().to_string());
    }
}
impl Default for Atom {
    fn default() -> Self { return Atom(DEBUG_DEFAULT.to_string()) }
}

#[derive(Debug)]
pub struct Sequence(Vec<Statement>);
impl Default for Sequence { 
    fn default() -> Self { Sequence(Vec::new()) }
}

#[derive(Debug)]
pub struct UnaryMinus {
    pub statement : Box<Statement>
}
#[derive(Debug)]
pub struct BinaryOperator {
    lhs: Box<Statement>,
    op: Operator,
    rhs: Box<Statement>,
}

pub enum Statement {
    Goal(Goal),
    Method(Method),
    Variable(Variable),
    Atom(Atom),
    Integer(i32),
    Float(f32),
    Sequence(Sequence),
    // expressions
    UnaryMinus(UnaryMinus),
    BinaryOperator(BinaryOperator),
    Empty, 
}


impl std::fmt::Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Statement::Goal(x)          => write!(f, "{:?}", x),
            Statement::Method(x)        => write!(f, "{:?}", x),
            Statement::Variable(x)      => write!(f, "{:?}", x),
            Statement::Atom(x)          => write!(f, "{:?}", x),
            Statement::Integer(x)       => write!(f, "Integer({})", x),
            Statement::Float(x)         => write!(f, "Float({})", x),
            Statement::Sequence(x)      => write!(f, "{:?}", x),
            Statement::Empty            => write!(f, "EMPTY"),
            Statement::UnaryMinus(x)    => write!(f, "{:?}", x),
            Statement::BinaryOperator(x)=> write!(f, "{:?}", x),
            rule => unreachable!("unexpected rule: {:?}", rule),
        }
        
    }
}

impl Parse for Statement {
    fn parse(pair : Pair<AstRule>) -> Self {
        match pair.as_rule() {
            AstRule::goal => Statement::Goal(Goal::parse(pair)),
            AstRule::head => Statement::Method(Method::parse(pair)),
            AstRule::variable => Statement::Variable(Variable::parse(pair)),
            AstRule::atom => Statement::Atom(Atom::parse(pair)),
            AstRule::signed_integer => Statement::Integer(pair.as_str().parse::<i32>().unwrap()),
            AstRule::signed_float => Statement::Float(pair.as_str().parse::<f32>().unwrap()),
            AstRule::expr => parse_expr(pair.into_inner()), 
            AstRule::EOI => Statement::Empty,
            rule => unreachable!("unexpected rule: {:?}", rule),
        }


        
    }
}

impl Default for Statement {
    fn default() -> Self { Statement::Empty }
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equals,
    GT,
    LT,
    GTE,
    LTE,
    Pipe,
}


pub fn parse_expr(pairs: Pairs<AstRule>) -> Statement {
    PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            AstRule::expr => parse_expr(primary.into_inner()),
            AstRule::signed_integer => Statement::Integer(primary.as_str().parse::<i32>().unwrap()),
            AstRule::signed_float => Statement::Float(primary.as_str().parse::<f32>().unwrap()),
            AstRule::variable => Statement::Variable(Variable::parse(primary)),
            rule => unreachable!("{:?}", rule)
        })
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                AstRule::add => Operator::Add,
                AstRule::subtract => Operator::Subtract,
                AstRule::multiply => Operator::Multiply,
                AstRule::divide => Operator::Divide,
                AstRule::modulo => Operator::Modulo,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Statement::BinaryOperator(BinaryOperator {  lhs: Box::new(lhs), op, rhs: Box::new(rhs)})
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            AstRule::minus => Statement::UnaryMinus(UnaryMinus { statement : Box::new(rhs) } ),
            _ => unreachable!(),
        })
        .parse(pairs)
}


