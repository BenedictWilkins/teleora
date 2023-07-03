use std::{collections::HashMap};
use crate::statement::{Statement, Variable, UList, List, AsStatement};
use crate::statement::collection::Collection;
use crate::statement::perm::PermutationIterator;

impl Frame {

    //pub fn evaluate_sequence(seq1: & Collection<& [Statement]>, seq2: & Collection<&[Statement]>, frame: & Frame)  -> Box<dyn Iterator<Item = Frame> > {
    pub fn evaluate_sequence(seq1: &Collection<& [Statement]>, seq2: & Collection<& [Statement]>, frame: &Frame)  -> Box<dyn Iterator<Item = Frame>> {
        // evaluate the first elements of seq1 and seq2. This should produce an iterator over Frame objects.
        // the Frames should contain any variables which were grounded during the evaluation.
        if seq1.len() != seq2.len() {
            println!("Failed to match, lengths: {:?} != {:?}", seq1.len(), seq2.len()); 
            return empty(); // this failed. TODO debug...
        }
        return FrameGenerator::new(seq1.items, seq2.items, singleton(frame.clone()));
    }   

    pub fn evaluate_ulist(l1 : &UList, l2 : &UList, frame : &Frame) -> Box<dyn Iterator<Item = Frame>> {
        // ulists are hard to deal with efficiently as permuations can be expensive. There are common special cases to consider:
        // e.g. {A|_} = {1,2} which will produce 2 frames A=1, A=2.

        if !l1.ispiped && !l2.ispiped { // simplest case
            // TODO for now check that one side is grounded
            let l1items = l2.items.as_ref();
            let mut l2items = l1.items.as_ref().to_vec().clone(); // TODO any better way that doesnt need a copy?
            
            println!("{:?} --- {:?}", l1items.items, l2items.items);

            let permiter = PermutationIterator::new(l1items.items, &mut l2items.items);
            

            for p in permiter {
                println!("??{:?}", p);
            }


            
            // {A,A} = {1,2} -> fails, but how to do this efficiently?

            // {A,A} = {B,1} // ungrounded variable... fail.
            // {A,A} = {A,1} // succeed?

        }


        // {A|B} = {1|[2]}  ?? TODO what is this behaviour...
        // {A|B} = {1|{2}}  // recall that {A|{B}} = {A,B} in this case it may be better to unpack {1|{2}} 
        
        return empty();
    }

    pub fn evaluate_list(l1: & List, l2: &List, frame: &Frame)  -> Box<dyn Iterator<Item = Frame>> {
        // check if the lists are piped...
        //println!("--{:?}", l1);
        //println!("--{:?}", l2);
        if (l1.ispiped && l2.ispiped) || (!l1.ispiped && !l2.ispiped) {
            // evaluate these like a sequence :) 
            return Frame::evaluate_sequence(&l1.items.as_ref(), &l2.items.as_ref(), frame);
        } else if l1.ispiped && !l2.ispiped {
            let piped_size = l1.len() - 1;
            if piped_size > l2.len() {
                println!("Failed to match, lengths: {:?} != {:?}", piped_size, l2.len()); // TODO debug
                return empty(); // fail.
            }
            let l1items = l1.items.as_ref();
            let l2items = l2.items.as_ref();
            let (l1x, l1p) = l1items.split_at(piped_size); 
            let (l2x, l2p) = l2items.split_at(piped_size);
            let xiter = Frame::evaluate_sequence(&l1x, &l2x, frame);
            let l2plist = List::new(l2p.to_vec(), false).as_statement(); // TODO this could also potentially be a UList?
            return FrameGenerator::new_singlular(&l1p[0], &l2plist, xiter);
        } else if !l1.ispiped && l2.ispiped {
            let piped_size = l2.len() - 1;
            if piped_size > l1.len() {
                println!("Failed to match, lengths: {:?} != {:?}", l1.len(), piped_size); // TODO debug
                return empty(); // fail.
            }
            let l1items = l1.items.as_ref();
            let l2items = l2.items.as_ref();
            let (l1x, l1p) = l1items.split_at(piped_size); 
            let (l2x, l2p) = l2items.split_at(piped_size);
            let xiter = Frame::evaluate_sequence(&l1x, &l2x, frame);
            let l1plist = List::new(l1p.to_vec(), false).as_statement(); // TODO this could also potentially be a UList?
            return FrameGenerator::new_singlular(&l1plist, &l2p[0], xiter);
        } 
        unreachable!();
    }   

    pub fn evaluate_statement(statement1: &Statement, statement2 : &Statement, frame : &Frame) -> Box<dyn Iterator<Item = Frame>> {
        // evaluate the statements. this should return an iterator of Frame objects.
        // each frame Object should be combined with the current frame.
        let result = match (statement1, statement2) {
            (Statement::Atom(atom1),    Statement::Atom(atom2))          => Frame::evaluate_primitive(atom1, atom2),          // do nothing to the frame
            (Statement::Integer(int1),  Statement::Integer(int2))        => Frame::evaluate_primitive(int1, int2),            // do nothing to the frame
            (Statement::Float(float1),  Statement::Float(float2))        => Frame::evaluate_primitive(float1, float2),        // do nothing to the frame
            (Statement::Variable(var1), _)                               => Frame::evaluate_variable(var1, statement2, &frame),
            (_,                         Statement::Variable(var2))       => Frame::evaluate_variable(var2, statement1, &frame),
            (Statement::List(l1),       Statement::List(l2))             => Frame::evaluate_list(l1, l2, &frame),
            (Statement::UList(l1),      Statement::UList(l2))            => Frame::evaluate_ulist(l1, l2, &frame),

            (s1, s2) => {println!("Failed to match {:?} {:?}", s1, s2); return empty()}
        };
        return result;
    } 

    
    fn evaluate_variable(variable: &Variable, statement: &Statement, frame : &Frame) -> Box<dyn Iterator<Item = Frame>> {
        if variable.is_anonymous() {
            return singleton(Frame::new()); // nothing changes...
        } else if let Some(value) = frame.get(variable) {
           if value == statement {
                return singleton(Frame::new()); // nothing changes...
            } else if let Statement::Variable(othervar) = value {
                let iter = Frame::evaluate_variable(othervar, statement, frame);
                

                return empty();
            } else {
                println!("Variable {:?} was already grounded {:?} but didn't match {:?}", variable, value, statement);
                return empty();
           }
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

pub fn empty<'a, T>() -> Box<dyn Iterator<Item = T> + 'a> where T : 'a {
    let v: Vec<T> = Vec::new();
    Box::new(v.into_iter())
}

pub fn singleton<'a, T>(value : T) -> Box<dyn Iterator<Item=T> + 'a> where T : 'a {
    let mut v = Vec::<T>::new();
    v.push(value);
    return Box::new(v.into_iter());
}

// used to match sequences
pub struct FrameGenerator {
    parent_frames : Box<dyn Iterator<Item=Frame>>,
    current_parent_frame : Option<Frame>,
    frames : Box<dyn Iterator<Item=Frame>>,
    x1: Statement,
    x2: Statement,
}

impl FrameGenerator {

    pub fn new_singlular(s1 : &Statement, s2 : &Statement, mut parent_frames : Box<dyn Iterator<Item=Frame>>) -> Box<dyn Iterator<Item=Frame>> {
        let current_parent_frame = parent_frames.next();
        if current_parent_frame.is_none() { // no parent frame was avaliable... a match failed immediately 
            return empty();
        }
        let current_parent_frame_unwrapped = current_parent_frame.unwrap();

        let frames = Frame::evaluate_statement(&s1, &s2, &current_parent_frame_unwrapped);
        
        return Box::new(FrameGenerator { x1 : s1.clone(), x2 : s2.clone(), 
                            current_parent_frame : Some(current_parent_frame_unwrapped), 
                            frames : frames, 
                            parent_frames : parent_frames });  
    }

    pub fn new(seq1: &[Statement], seq2: &[Statement], mut parent_frames : Box<dyn Iterator<Item=Frame>>) -> Box<dyn Iterator<Item=Frame>> {
        if seq1.len() == 0 || seq2.len() == 0 {
            panic!("Invalid use of FrameGenerator, sequences must not be empty.");
        }
        if seq1.len() != seq2.len() {
            panic!("Invalid use of FrameGenerator, sequences must be the same length: {:?} != {:?}", seq1.len(), seq2.len());
        }
        
        let (x1, s1) = seq1.split_at(1);
        let (x2, s2) = seq2.split_at(1);

        let current_parent_frame = parent_frames.next();
        if current_parent_frame.is_none() { // no parent frame was avaliable... a match probably failed immediately 
            return empty();
        }
        let current_parent_frame_unwrapped = current_parent_frame.unwrap();

        let frames = Frame::evaluate_statement(&x1[0], &x2[0], &current_parent_frame_unwrapped);
        
        let current = Box::new(FrameGenerator { x1 : x1[0].clone(), x2 : x2[0].clone(), 
                                                current_parent_frame : Some(current_parent_frame_unwrapped), 
                                                frames : frames, 
                                                parent_frames : parent_frames });            
        if s1.len() > 0 { 
            return FrameGenerator::new(s1, s2, current);
        } else {
            return current; // end of sequence reached.
        }
    }
}

impl Iterator for FrameGenerator {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
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
        //println!("--{:?}", frame_unwrapped);
        return Some(frame_unwrapped);
    }
}







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
