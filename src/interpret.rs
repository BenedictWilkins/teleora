
use itertools::Itertools;
use pest::pratt_parser::{Assoc::*, Op, PrattParser};
use pest::iterators::{Pair, Pairs};
use crate::Rule as AstRule;

use crate::statement::{Statement, Variable, Atom, Sequence, UnaryOperator, BinaryOperator, Integer, Float, AsStatement, List, UList, Object};

// pratt parser to handle operator prescendence
lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<AstRule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use AstRule::*;
        // Precedence is defined lowest to highest
        PrattParser::<AstRule>::new()
            //.op(Op::infix(pipe, Left))
            .op(Op::infix(eq, Left) | Op::infix(lt, Left) | Op::infix(gt, Left) | Op::infix(lte, Left) | Op::infix(gte, Left))
            .op(Op::infix(or, Left) | Op::infix(and, Left))
            .op(Op::infix(add, Left) | Op::infix(subtract, Left))
            .op(Op::infix(multiply, Left) | Op::infix(divide, Left) | Op::infix(modulo, Left))
            .op(Op::prefix(minus) | Op::prefix(not))
    };
}

#[derive(Debug, PartialEq)]
pub struct Action;

#[derive(Debug, PartialEq)]
pub struct Condition;

#[derive(Debug, PartialEq)]
pub struct Rule { 
    conditions : Vec<Condition>,
    actions : Vec<Action>,
}

#[derive(Debug)]
pub struct Head {
    name : Atom, 
    arguments : Sequence,
}
impl PartialEq for Head {
    fn eq(&self, other: &Self) -> bool { self.name == other.name }
}
impl Default for Head { 
    fn default() -> Self { return Head { name : Atom::default(), arguments : Sequence::default() } }
}

#[derive(Debug, PartialEq)]
pub struct Goal {
    head : Head, 
    body : Vec<Rule>,
}
impl Default for Goal { 
    fn default() -> Self { return Goal { head : Head::default(), body : Vec::new() } }
}

pub fn interpret(mut document: Pairs<AstRule>) -> Goal {
    let goal_rules = document.next().unwrap().into_inner();
    let glen = goal_rules.len();
    let goals = goal_rules.into_iter().take(glen - 1)
        .map(|goal| Goal::new(goal))
        .group_by(|goal| goal.get_name())
        .into_iter()
        .map(|(name, group)| (name, group.collect()))
        .collect::<Vec<(String, Vec<Goal>)>>();
    println!("???? {:?}", goals);
    return Goal::default();
}

pub fn interpret_expression(expr : Pairs<AstRule>) -> Statement {
    //println!("    {:?}", expr);
    let result = PRATT_PARSER
        .map_primary(|primary| match primary.as_rule() {
            AstRule::expr               => interpret_expression(primary.into_inner()),
            AstRule::signed_integer     => Integer::new(primary).as_statement(),
            AstRule::signed_float       =>  Float::new(primary).as_statement(),
            AstRule::variable           => Variable::new(primary).as_statement(),
            AstRule::atom               => Atom::new(primary).as_statement(),
            AstRule::list               => List::new(primary).as_statement(),
            AstRule::ulist              => UList::new(primary).as_statement(),
            AstRule::seq                => Sequence::new(primary).as_statement(),
            rule => unreachable!("{:?}", rule)
        })
        .map_infix(|lhs, op, rhs| {
            match op.as_rule() {
                AstRule::eq         => Statement::BinaryOperator(BinaryOperator::Equal(Box::new(lhs), Box::new(rhs))),
                AstRule::gt         => Statement::BinaryOperator(BinaryOperator::GreaterThan(Box::new(lhs), Box::new(rhs))),
                AstRule::gte        => Statement::BinaryOperator(BinaryOperator::GreaterThanEqual(Box::new(lhs), Box::new(rhs))),
                AstRule::lt         => Statement::BinaryOperator(BinaryOperator::LessThan(Box::new(lhs), Box::new(rhs))),
                AstRule::lte        => Statement::BinaryOperator(BinaryOperator::LessThanEqual(Box::new(lhs), Box::new(rhs))),
                AstRule::and        => Statement::BinaryOperator(BinaryOperator::And(Box::new(lhs), Box::new(rhs))),
                AstRule::or         => Statement::BinaryOperator(BinaryOperator::Or(Box::new(lhs), Box::new(rhs))),
                AstRule::add        => Statement::BinaryOperator(BinaryOperator::Add(Box::new(lhs), Box::new(rhs))),
                AstRule::subtract   => Statement::BinaryOperator(BinaryOperator::Subtract(Box::new(lhs), Box::new(rhs))),
                AstRule::multiply   => Statement::BinaryOperator(BinaryOperator::Multiply(Box::new(lhs), Box::new(rhs))),
                AstRule::divide     => Statement::BinaryOperator(BinaryOperator::Divide(Box::new(lhs), Box::new(rhs))),
                AstRule::modulo     => Statement::BinaryOperator(BinaryOperator::Modulo(Box::new(lhs), Box::new(rhs))),
                rule => unreachable!("Expected infix operation, found {:?}", rule),
            }
        })
        .map_prefix(|op, rhs| {
            match op.as_rule() {
                AstRule::minus  => Statement::UnaryOperator(UnaryOperator::Minus(Box::new(rhs))),
                AstRule::not    => Statement::UnaryOperator(UnaryOperator::Not(Box::new(rhs))),
                rule => unreachable!("Expected prefix operation, found {:?}", rule),
            }
        })
        .parse(expr);
    return result;
}

impl Goal { 
    pub fn new(pair : Pair<AstRule>) -> Goal { 
        //println!("----> {:?}", rule);
        let mut pairsinner = pair.into_inner();
        let head : Head = Head::new(pairsinner.next().unwrap()); // head can only take the form of a method
        let body : Vec<Rule> = pairsinner.map(|p| Rule::new(p)).collect(); 
        return Goal { head : head, body : body };
    }

    pub fn get_name(&self) -> String {
        return self.head.name.0.clone();
    }
}

impl Action { 
    pub fn new(pair : Pair<AstRule>) -> Action { 
        //println!("----> {:?}", rule);
        let mut pairsinner = pair.into_inner();
        return Action;
    }
}

impl Condition { 
    pub fn new(pair : Pair<AstRule>) -> Condition { 
       
        let condition = interpret_expression(pair.into_inner());
        println!("----> {:?}", condition);
        // evaluate this condition if possible...

        return Condition;
       
    }
}

impl Rule {
    pub fn new(pair : Pair<AstRule>) -> Rule { 
        //println!("----> {:?}", rule);
        let mut pairsinner = pair.into_inner();
        let conditions : Vec<Condition> = pairsinner.next().unwrap().into_inner().map(|p| Condition::new(p)).collect();
        let actions : Vec<Action> = pairsinner.next().unwrap().into_inner().map(|p| Action::new(p)).collect();
        return Rule { conditions : conditions, actions : actions};
    }
}

impl Head {
    pub fn new(pair : Pair<AstRule>) -> Head {
        let mut pairsinner = pair.into_inner();
        let name = Atom(pairsinner.next().unwrap().as_str().to_string()); // this should be an atom... 
        let arguments : Vec<Statement> = pairsinner.into_iter().map(|arg| interpret_expression(arg.into_inner())).collect();
        return Head { name : name, arguments : Sequence(arguments)};
    }
}

// implement interpret for each Statement type.
impl List { 
    pub fn new(pair : Pair<AstRule>) -> Self {
        let mut pairsinner = pair.into_inner();
        if let Some(lhead) = pairsinner.next() {
            let mut seq = match lhead.as_rule() {
                AstRule::seq => Sequence::new(lhead),
                rule => unreachable!("Expected sequence, found {:?}", rule),
            };
            if let Some(ltail) = pairsinner.next() {
                let tail = interpret_expression(ltail.into_inner());
                seq.0.push(tail);
                return List((seq, true));
            } 
            return List((seq, false));
        }
        return List::default(); // empty
    }
}

impl UList { 
    pub fn new(pair : Pair<AstRule>) -> Self {
        return List::new(pair).into();
    }
}

impl Sequence { 
    pub fn new(pairs : Pair<AstRule>) -> Self {
        let result : Vec<Statement> = pairs.into_inner().map(|p| interpret_expression(p.into_inner())).collect();
        return Sequence(result);
    }
}

impl Variable { 
    pub fn new(pair : Pair<AstRule>) -> Self {
        let name = pair.as_str().to_string();
        return Variable(name);
    }
}

impl Atom { 
    pub fn new(pair : Pair<AstRule>) -> Self {
        let name = pair.as_str().to_string();
        return Atom(name);
    }
}

impl Integer { 
    pub fn new(pair : Pair<AstRule>) -> Self {
        let value = pair.as_str().parse::<i32>().unwrap();
        return Integer(value);
    }
}

impl Float { 
    pub fn new(pair : Pair<AstRule>) -> Self {
        let value = pair.as_str().parse::<f32>().unwrap();
        return Float(value);
    }
}





