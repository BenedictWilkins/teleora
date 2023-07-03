

extern crate permutohedron;

use permutohedron::Heap;


use super::{Statement, Frame, frame::empty};

pub struct PermutationIterator<'a> {
    list1 : Vec<Statement>,
    heap: Heap<'a, &'a Vec<Statement>, Statement>,
}

impl<'a> PermutationIterator<'a> {

    pub fn new(list1: &'a [Statement], list2: &'a [Statement]) -> Self {
        if list1.len() != list2.len() {
            panic!("PermuationIterator requires lists that are equal in length.");
        }


        //pub fn new(data: &'a mut Data) -> Self
        let l1 = list1.to_vec();
        let l2 = list2.to_vec();        

        return PermutationIterator {
            list1 : l1,
            list2 : l2,
            heap : Heap::new(mut l2),
        }
    }
    
}

impl<'a> Iterator for PermutationIterator<'a> {
    type Item = (Vec<Statement>, Vec<Statement>);
    fn next(&mut self) -> Option<Self::Item> {
        let mut heap = &self.heap;

        

       //if let Some(heap) = self.heap {
            //if let Some(permutation) = heap.next_permutation() {
            //    return Some((permutation.to_vec(), self.list1.to_vec()))
            //} else {
            //    return None;
            //}
        //}
        return None;
    }
}

pub struct PermutationFrameIterator {
    parent_frame : Box<Frame>,
    permutation_iter : Box<dyn Iterator<Item=(Vec<Statement>,Vec<Statement>)>>,
    frame_iter : Box<dyn Iterator<Item=Frame>>,
}

impl PermutationFrameIterator {

    pub fn new(list1 : Vec<Statement>, list2 : Vec<Statement>, frame : &Frame) -> Box<dyn Iterator<Item=Frame>> {
        


        


       
        //let frame_iter = Frame::evaluate_sequence(first)

        //return Box::new(PermutationFrameIterator { parent_frame: Box::new(frame.clone()), 
        //                                 permutation_iter: permutation_iter,
        //                                  frame_iter : empty()});

        return empty()

    }
}

impl Iterator for PermutationFrameIterator {
    type Item = Frame;
    fn next(&mut self) -> Option<Self::Item> {
        let permutation = self.permutation_iter.next();
        // convert permutation list to new Frame
        if !permutation.is_none() {
            return Some(Frame::new());
            
        } else {
            return None;
        }

       
    }

}
