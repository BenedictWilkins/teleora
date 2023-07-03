
use pest::Parser;
use pest_derive::Parser;

#[macro_use]
mod utils;

mod interpret;
use interpret::{Program};

mod statement;

#[derive(Parser)]
#[grammar = "teleora.pest"]
pub struct TeleoraParser;

const TEST_PROGRAM:&str = include_str!("../test/test1.tela");
const TEST_OBSERVATION:&str = include_str!("../test/observation.tela");

fn main() {
    println!("{}", TEST_PROGRAM);
    println!("{}", TEST_OBSERVATION);
    match TeleoraParser::parse(Rule::document, TEST_PROGRAM) {
        
        Ok(pairs) => {
            let program = Program::new(pairs);
            if let Ok(observation) = TeleoraParser::parse(Rule::observation, TEST_OBSERVATION) {
                //println!("Observation: {:?}", observation);
                let sequence = Program::interpret_observation(observation).unwrap();
                //println!("Observation: {:?}", sequence);
                program.evaluate(sequence);
            } else {
                println!("Observation: {TEST_OBSERVATION} parse failed."); // TODO some info please.
            }
        }
        Err(e) => { println!("Parse failed: {:?}", e);}
    }
}



/*
fn evaluate(entry_point : Goal, arguments : Vec<Statement> ) -> Option<Action> {
    println!("{}", entry_point.get_name());

    // match arguments with the goal
   
    entry_point.evaluate(arguments); 
    
    return Option::None;

} */





