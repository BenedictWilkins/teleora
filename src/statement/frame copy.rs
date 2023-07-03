use std::{collections::HashMap};
use crate::statement::{Statement, Variable, Sequence, Atom, UList, List};

use super::collection::Collection;

impl Frame {

    //pub fn evaluate_sequence(seq1: & Collection<& [Statement]>, seq2: & Collection<&[Statement]>, frame: & Frame)  -> Box<dyn Iterator<Item = Frame> > {
    pub fn evaluate_sequence<'a>(seq1: &'a Collection<&'a [Statement]>, seq2: &'a Collection<&'a [Statement]>, frame: &'a Frame)  -> Box<dyn Iterator<Item = Frame> + 'a> {
        // evaluate the first elements of seq1 and seq2. This should produce an iterator over Frame objects.
        // the Frames should contain any variables which were grounded during the evaluation.
        if seq1.len() != seq2.len() {
            return empty(); // this failed.
        }
        let iter = FrameGenerator::new(seq1.items, seq2.items, singleton(frame.clone()));
        return iter;
    }   

    pub fn evaluate_list<'a>(l1: &'a List, l2: &'a List, frame: &'a Frame)  -> Box<dyn Iterator<Item = Frame> + 'a> {
        // evaluate the first elements of seq1 and seq2. This should produce an iterator over Frame objects.
        // the Frames should contain any variables which were grounded during the evaluation.
        if l1.len() != l2.len() {
            return empty(); // this failed.
        }
        let iter = FrameGenerator::new(l1.items.as_ref().items, l2.items.as_ref().items, singleton(frame.clone()));
        return iter;
    }   

    pub fn evaluate_statement<'a>(statement1: &Statement, statement2 : &Statement, frame : &Frame) -> Box<dyn Iterator<Item = Frame>> {
        // evaluate the statements. this should return an iterator of Frame objects.
        // each frame Object should be combined with the current frame.
        let result = match (statement1, statement2) {
            (Statement::Atom(atom1),    Statement::Atom(atom2))          => Frame::evaluate_primitive(atom1, atom2),          // do nothing to the frame
            (Statement::Integer(int1),  Statement::Integer(int2))        => Frame::evaluate_primitive(int1, int2),            // do nothing to the frame
            (Statement::Float(float1),  Statement::Float(float2))        => Frame::evaluate_primitive(float1, float2),        // do nothing to the frame
            (Statement::Variable(var1), _)                               => Frame::evaluate_variable(var1, statement2, &frame),
            (_,                         Statement::Variable(var2))       => Frame::evaluate_variable(var2, statement1, &frame),
            //(Statement::List(l1),       Statement::List(l2))             => Frame::evaluate_list(l1, l2, &frame),
            
            (s1, s2) => {println!("Failed to match {:?} {:?}", s1, s2); return empty()}
        };
        return result;
    } 

    
    fn evaluate_variable(variable: &Variable, statement: &Statement, frame : &Frame) -> Box<dyn Iterator<Item = Frame>> {
        if let Some(value) = frame.get(variable) {
           return Frame::evaluate_primitive(value, statement);
        } else {
            let mut new_frame = Frame::new();
            new_frame.insert(variable, statement.clone()); // TODO we are assuming that the statement is grounded...
            return singleton(new_frame);
        }
    }

    /// this matches primitive types
    fn evaluate_primitive<T : PartialEq + std::fmt::Debug>(arg1 : &T, arg2 : &T) -> Box<dyn Iterator<Item = Frame>>{
        if arg1 == arg2 {
            return singleton(Frame::new());
        } else {
            println!("Failed to match {:?} {:?}", arg1, arg2); // TODO better debug info
            return empty();
        } 
    } 
}

fn empty<'a, T>() -> Box<dyn Iterator<Item = T> + 'a> where T : 'a {
    let v: Vec<T> = Vec::new();
    Box::new(v.into_iter())
}

fn singleton<'a, T>(value : T) -> Box<dyn Iterator<Item=T> + 'a> where T : 'a {
    let mut v = Vec::<T>::new();
    v.push(value);
    return Box::new(v.into_iter());
}

// used to match sequences
struct FrameGenerator<'a> {
    parent_frames : Box<dyn Iterator<Item=Frame> + 'a>,
    current_parent_frame : Option<Frame>,
    frames : Box<dyn Iterator<Item=Frame> + 'a>,
    x1: Statement,
    x2: Statement,
}

impl<'a> FrameGenerator<'a> {

    pub fn new(seq1: &'a [Statement], seq2: &'a [Statement], mut parent_frames : Box<dyn Iterator<Item=Frame> + 'a>) -> Box<dyn Iterator<Item=Frame> + 'a> {
        if seq1.len() != seq2.len() {
            return empty(); // this failed.
        }
    
        let (x1, s1) = seq1.split_at(1);
        let (x2, s2) = seq2.split_at(1);

        
        // this is the iterator for the first elements sequence, it should be used to generate frames for the next elements
        //let iter = Frame::evaluate_statement(&x1[0], &x2[0], frame);

        let current_parent_frame = parent_frames.next();
        if current_parent_frame.is_none() { // no parent frame was avaliable... a match probably failed immediately 
            return empty();
        }

        let frames = Frame::evaluate_statement(&x1[0].clone(), &x2[0].clone(), current_parent_frame.as_ref().unwrap());
        let current = Box::new(FrameGenerator { x1 : x1[0].clone(), x2 : x2[0].clone(), 
                                                current_parent_frame : current_parent_frame, 
                                                frames : frames, 
                                                parent_frames : parent_frames });            
        if s1.len() > 0 { 
            return FrameGenerator::new(s1, s2, current);
        } else {
            return current; // end of sequence reached.
        }
    }
}

impl<'a> Iterator for FrameGenerator<'a> {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        //println!("??{:?} {:?}", self.x1, self.x2);
        let frame = self.frames.next();
        if frame.is_none() { // cannot produce any more frames with this parent frame, try next one.
            self.current_parent_frame = self.parent_frames.next();
            if self.current_parent_frame.is_none() {
                return None; // no more parent frames, cannot proceed.
            }
            // evaluate statement should also return some debug information? i.e. whether the match failed or not.
            self.frames = Frame::evaluate_statement(&self.x1, &self.x2, self.current_parent_frame.as_ref().unwrap());
            return self.next();
        }
        // join parent frame with this frame, these cannot be None
        let current_parent_frame_unwrapped = self.current_parent_frame.as_ref().unwrap();
        let mut frame_unwrapped = frame.unwrap();
        frame_unwrapped.join(current_parent_frame_unwrapped);
        println!("--{:?}", frame_unwrapped);
        return Some(frame_unwrapped);
    }
}


/*/


pub struct FuncGenerator<T, F> where F: FnMut() -> Option<T> {
    state: F,
}

impl<T, F> FuncGenerator<T, F> where F: FnMut() -> Option<T> {
    pub fn new(state: F) -> Self {
        FuncGenerator { state }
    }

    pub fn singleton(value : T) -> FuncGenerator<T, impl FnMut() -> Option<T>> {
        FuncGenerator { state:  singleton(value) }
    }

    pub fn none() -> FuncGenerator<T, impl FnMut() -> Option<T>> {
        FuncGenerator { state:  || None }
    }
}

// intended for use with FuncGenerator::singleton
fn singleton<T>(value : T) -> impl FnMut() -> Option<T> {
    let mut vec = Vec::<T>::new();
    vec.push(value);
    move || {
        return vec.pop();
    }
}


impl<T, F> Iterator for FuncGenerator<T, F> where F: FnMut() -> Option<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        (self.state)()
    }
}

fn unify_statement(statement1: &Statement, statement2: &Statement, frame : &Frame)  -> FuncGenerator<Frame, impl FnMut() -> Option<Frame>> {
//fn unify_statement(statement1: &Statement, statement2: &Statement, frame : &Frame) -> Box<dyn Iterator<Item = Frame>> {
    //let mut count = 0;
    //let mut gen2 = FuncGenerator::new(count_down());
    let result = match (statement1, statement2) {
        (Statement::Variable(var1), _)                               => Frame::unify_variable(var1, statement2, frame),
        (_,                         Statement::Variable(var2))       => Frame::unify_variable(var2, statement1, frame),
        //(Statement::Atom(atom1),    Statement::Atom(atom2))          => Frame::unify_primitive(atom1, atom2),          // do nothing to the frame
        //(Statement::Integer(int1),  Statement::Integer(int2))        => Frame::unify_primitive(int1, int2),            // do nothing to the frame
        //(Statement::Float(float1),  Statement::Float(float2))        => Frame::unify_primitive(float1, float2),        // do nothing to the frame
        //(Statement::Sequence(seq1),  Statement::Sequence(seq2))      => FrameGenerator::unify_sequence(seq1, seq2, frame),
        //(Statement::Compound(comp1), Statement::Compound(comp2)) => unify_compound(comp1, comp2, frame),
        //(Statement::UList(list1), Statement::UList(list2)) => Frame::unify_ulist(list1, list2),
        _ => Box::new(FuncGenerator::new(|| None)), // unification failed.
    };

    FuncGenerator::new(|| None)
}
}

pub fn unify_ulist<'a>(list1: &UList, list2: &UList, frame: Frame) -> Box<dyn Iterator<Item = Frame>>  {
    // try matching the first elements
    if (list1.ispiped && list2.ispiped) || (!list1.ispiped && !list2.ispiped) {
        if list1.len() != list2.len() {
            return Box::new(FuncGenerator::new(|| None)); // lists have different lengths, they dont match
        }
        // try matching each element, make copies of Frame


    } else if list1.ispiped && !list2.ispiped {
        // this first list is piped, so we need to match the last N items of list2 with the last element of list1
        let dif = (list2.len() as i32) - (list1.len() as i32);
        if dif < 0 {
            return Box::new(FuncGenerator::new(|| None)); // list 2 was not long enough
        }



    } else {
        return Frame::unify_ulist(list2, list1, frame); // swap the lists.
    }
    return Box::new(FuncGenerator::new(|| None));
}

/// this matches primitive types and leaves the given frame unchanged.
fn unify_primitive<T : PartialEq>(arg1 : &T, arg2 : &T, frame : Frame) -> Box<dyn Iterator<Item = Frame>> {
    if arg1 == arg2 {
        //let generator = FuncGenerator::singleton(frame);
        return Box::new(FuncGenerator::new(singleton(frame)));
    } else {
        return Box::new(FuncGenerator::new(|| None));
    } 
} 

fn unify_variable(variable: &Variable, statement: &Statement, frame : &Frame) -> Box<dyn Iterator<Item = Frame>> {
    
    
    if let Some(value) = frame.get(variable) {
        Frame::unify_statement(value, statement, frame); 
        
    } else {
        //frame.insert(variable, statement.clone());
        //return Some(frame);
    }
   

    Box::new(FuncGenerator::new(move || {
        return None;
    }))
}
fn unify_statement(statement1: &Statement, statement2: &Statement, frame : &Frame)  -> FuncGenerator<Frame, impl FnMut() -> Option<Frame>> {
//fn unify_statement(statement1: &Statement, statement2: &Statement, frame : &Frame) -> Box<dyn Iterator<Item = Frame>> {
    //let mut count = 0;
    //let mut gen2 = FuncGenerator::new(count_down());
    let result = match (statement1, statement2) {
        (Statement::Variable(var1), _)                               => Frame::unify_variable(var1, statement2, frame),
        (_,                         Statement::Variable(var2))       => Frame::unify_variable(var2, statement1, frame),
        //(Statement::Atom(atom1),    Statement::Atom(atom2))          => Frame::unify_primitive(atom1, atom2),          // do nothing to the frame
        //(Statement::Integer(int1),  Statement::Integer(int2))        => Frame::unify_primitive(int1, int2),            // do nothing to the frame
        //(Statement::Float(float1),  Statement::Float(float2))        => Frame::unify_primitive(float1, float2),        // do nothing to the frame
        //(Statement::Sequence(seq1),  Statement::Sequence(seq2))      => FrameGenerator::unify_sequence(seq1, seq2, frame),
        //(Statement::Compound(comp1), Statement::Compound(comp2)) => unify_compound(comp1, comp2, frame),
        //(Statement::UList(list1), Statement::UList(list2)) => Frame::unify_ulist(list1, list2),
        _ => Box::new(FuncGenerator::new(|| None)), // unification failed.
    };

    FuncGenerator::new(|| None)
}








*/


#[derive(Debug, Clone)]
pub struct Frame {
    map: HashMap<String, Statement>, // value of variables
}

impl Frame {
    // Create a new instance of Frame
    pub fn new() -> Self {
        Frame {
            map: HashMap::new(),
        }
    }

    pub fn join(&mut self, other : &Frame) {
        for (key, value) in other.map.iter() {
            self.map.insert(key.clone(), value.clone());
        }
    }

    // Insert a key-value pair into the map
    pub fn insert(&mut self, key: &Variable, value: Statement) {
        self.map.insert(key.name.clone(), value);
    }

    // Get the value associated with a key
    pub fn get(&self, key: &Variable) -> Option<&Statement> {
        return self.map.get(key.name.as_str());
    }

    // Remove a key-value pair from the map
    pub fn remove(&mut self, key: &Variable) -> Option<Statement> {
        self.map.remove(&key.name)
    }

    // Check if the map is empty
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    // Get the number of elements in the map
    pub fn len(&self) -> usize {
        self.map.len()
    }

    // Clear all key-value pairs from the map
    pub fn clear(&mut self) {
        self.map.clear();
    }
}




impl Default for Frame {
    fn default() -> Self {
        Self { map: Default::default() }
    }
}
