use pest::pratt_parser::{Assoc::*, Op, PrattParser};
use pest::iterators::{Pair, Pairs};
use crate::Rule as AstRule;
use crate::frame::Frame;

pub static DEBUG_DEFAULT : &str = "debug_default";

// pratt parser to handle operator prescendence
lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<AstRule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use AstRule::*;
        // Precedence is defined lowest to highest
        PrattParser::<AstRule>::new()
            .op(Op::infix(eq, Left) | Op::infix(lt, Left) | Op::infix(gt, Left) | Op::infix(lte, Left) | Op::infix(gte, Left))
            .op(Op::infix(or, Left) | Op::infix(and, Left))
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
            .op(Op::prefix(minus) | Op::prefix(not))
    };
}

// implemented by each ast struct to pair from PEG parser
pub trait Parse {
    fn parse(pair : Pair<AstRule>) -> Self;
}

#[derive(Debug, PartialEq)]
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
impl Goal {
    pub fn evaluate(&self, arguments : Vec<Statement>) {
        if let Some(frame) = self.head.evaluate(&arguments) { // the head of this goal was successfully matched
            // now evaluate the body using frame.
            for rule in self.body.iter() {
                rule.evaluate(&frame); // this may fail, or return an action.
            }
        }
    }

    pub fn get_name(&self) -> String {
        return self.head.name.0.clone();
    }

    pub fn get_head(&self) -> &Method {
        return &self.head;
    }
}


#[derive(Debug, PartialEq)]
pub struct Rule { 
    pub conditions : Vec<Condition>,
    pub actions : Vec<Action>,
}
impl Rule {
    pub fn evaluate(&self, frame : &Frame) {
        println!("{:?}", self);
        let mut rframe : Frame = Frame::default();
        // evaluate each condition grounding variables along the way 
        let result : Vec<bool> = self.conditions.iter().map(|c| {
            return c.evaluate(frame, &mut rframe);


        }).collect();
    }
}
impl Default for Rule { 
    fn default() -> Self { return Rule { conditions : Vec::new(), actions : Vec::new() }; }
}
impl Parse for Rule {
    fn parse(pair : Pair<AstRule>) -> Self {
        let mut pairs = pair.into_inner(); // rules contain conditions -> actions
        //println!("{:?}", pairs);
        let conditions : Vec<Condition> = pairs.next().unwrap().into_inner().map(|p| Condition::parse(p)).collect();
        let actions : Vec<Action> = pairs.next().unwrap().into_inner().map(|p| Action::parse(p)).collect();
        return Rule { conditions : conditions, actions : actions};
    }
}

#[derive(Debug, PartialEq)]
pub struct Condition(Statement);
impl Condition {

    pub fn evaluate(&self, frame : &Frame, rframe : &mut Frame) -> bool {
        // frame and rframe are always disjoint.
        match &self.0 {
            Statement::Atom(_) => { return true; },
            Statement::BinaryOperator(x) => { x.op.evaluate(x.lhs, x.rhs) },
            //Statement::Variable(x) => { let _y = self.get_grounded(x, frame, rframe); return true; }, // TODO do we have a boolean type?
            //Statement::UnaryOperator(x) => { x.evaluate(frame, rframe) }
            rule => unreachable!("Expected Condition but found {:?}", rule)
        }
    }

    fn get_grounded<'p>(&self, var : &Variable, frame : &Frame<'p>, rframe : &Frame<'p>) -> &'p Statement {
        if let Some(y) = frame.variables.get(&var.name) {
            return y;
        } else if let Some(y) = rframe.variables.get(&var.name) {
            return y;
        } else {
            panic!("{:?} could not be grounded.", var); // TODO hmmm...
        }
    }
}
impl Default for Condition { 
    fn default() -> Self { return Condition(Statement::Empty); }
}

impl Parse for Condition {
    fn parse(pair : Pair<AstRule>) -> Self {
        //println!("{:?}", pair);
        return Condition(Statement::parse(pair));
    }
}

#[derive(Debug, PartialEq)]
pub struct Action(Statement);

impl Default for Action {
    fn default() -> Self { return Action(Statement::Empty); }
}
impl Parse for Action {
    fn parse(pair : Pair<AstRule>) -> Self {
        match pair.as_rule() {
            AstRule::head => Action(Statement::Method(Method::parse(pair))),
            AstRule::atom => Action(Statement::Atom(Atom::parse(pair))),
            // TODO colletions, numbers
            AstRule::variable => Action(Statement::Variable(Variable::parse(pair))),
            _ => unreachable!("Expected `action` or `subgoal` but received {pair}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Method {
    pub name : Atom,
    pub arguments : Sequence
}
impl Method {

    /// Gets the length of the arguments of this method.
    pub fn len(&self) -> usize { // length of
        return self.arguments.len();
    }

    /// evalates this method, returning a `Frame` object if argument matching succeeds for continued evaluation.
    pub fn evaluate<'p>(&self, arguments : &'p Vec<Statement>) -> Option<Frame<'p>> {
        let mut frame = Frame::default();

        // attempt to match arguments given to this method.
        // TODO handle variable length arguments...
        let (l1, l2) = (arguments.len(), self.arguments.len());
        if l1 != l2 { // incoming arguments length and header arguments length do not match, so fail. 
            return Option::None; // None indicates that matching failed, try the next instance. TODO update this so that some message can be given for debugging purposes.
        }
        // iterate and try matching
        let result : Vec<bool> = self.arguments.iter().zip(arguments.iter()).map(|(a,b)| {
            let matched = a.match_statement(b, &mut frame);
            println!("match {} {:?} {:?} {:?}", matched, a, b, frame);
            return matched;
        }).collect();
        let matched = result.iter().all(|&value| value); 
        if !matched { // some of the arguments did not match, so fail. 
            return Option::None; // TODO give more information about where the failure occured for debugging purposes
        }
        // `frame` should now be populated with the variables required to proceed with condition/action rule evaluation.
        return Option::Some(frame);
    }
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

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name : String,
}

impl Parse for Variable {
    fn parse(pair : Pair<AstRule>) -> Self {
        return Variable { name : pair.as_str().to_string() } ;
    }
}

#[derive(Debug, PartialEq)]
pub struct Atom(String);
impl Parse for Atom {
    fn parse(pair : Pair<AstRule>) -> Self {
        return Atom(pair.as_str().to_string());
    }
}
impl Default for Atom {
    fn default() -> Self { return Atom(DEBUG_DEFAULT.to_string()) }
}

#[derive(Debug, PartialEq)]
pub struct Sequence(Vec<Statement>);
impl Sequence {
    pub fn len(&self) -> usize {
        return self.0.len();
    }
    pub fn iter(&self) -> std::slice::Iter<'_, Statement> {
        self.0.iter()
    }
}
impl Default for Sequence { 
    fn default() -> Self { Sequence(Vec::new()) }
}

#[derive(Debug, PartialEq)]
pub struct UnaryOperator {
    rhs : Box<Statement>,
    op : Operator,
}

impl UnaryOperator {
    pub fn evaluate(&self, frame : &Frame, rframe : &Frame) {
        match &(*self.rhs) {
            Statement::Atom(x) => {},
            Statement::Variable(x) => {},
            Statement::Integer(x) => {},
            Statement::Float(x) => {},
            _ => unreachable!()
        }
    }

    fn eval_Not<T>(&self, frame : &Frame, rframe : &mut Frame) {}
    fn eval_Minus<T>(&self, frame : &Frame, rframe : &mut Frame) {}
}


#[derive(Debug, PartialEq)]
pub struct BinaryOperator {
    lhs: Box<Statement>,
    op: Operator,
    rhs: Box<Statement>,
}




#[derive(PartialEq)]
pub enum Statement {
    Goal(Goal),
    Method(Method),
    Variable(Variable),
    Atom(Atom),
    Integer(i32),
    Float(f32),
    Sequence(Sequence),
    // expressions
    BinaryOperator(BinaryOperator),
    UnaryOperator(UnaryOperator),
    Empty, 
}

impl Statement {
    
    // TODO refactor? move this somewhere more suitable?
    fn match_statement<'p> (&self, other : &'p Statement, frame : &mut Frame<'p>) -> bool {
        match self {
            Statement::Variable(x)  => {  // x is a variable, this means it needs to be added to `frame` for later use.
                // check if this variable already exists, if it does, then we need to check that the new value matches the old one.
                if let Some(old) = frame.variables.get(&x.name) {
                   if !(*old).eq(other) { return false; }
                } 
                frame.variables.insert(x.name.clone(), other); 
                return true; // variables always match
            },
            Statement::Atom(x)      => { let Some(y) = trycast!(other, Statement::Atom) else { return false; }; return y.eq(x) }, 
            Statement::Integer(x)   => { let Some(y) = trycast!(other, Statement::Integer) else { return false; }; return y.eq(x) },
            // TODO float approx. equal for pattern matching, have some means of setting this
            Statement::Float(x)     => { let Some(y) = trycast!(other, Statement::Float) else { return false; }; return y.eq(x) },
            _ => { return false },
        };
    }
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
            Statement::UnaryOperator(x) => write!(f, "{:?}", x),
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
            AstRule::lexpr => parse_expr(pair.into_inner()), 
            // AstRule::list // TODO
            AstRule::EOI => Statement::Empty,
            rule => unreachable!("unexpected rule: {:?}", rule),
        }
    }
}

impl Default for Statement {
    fn default() -> Self { Statement::Empty }
}

/* 
#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    Minus, // unary
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    And,
    Or,
    Not, // unary
    Pipe,
}
*/





