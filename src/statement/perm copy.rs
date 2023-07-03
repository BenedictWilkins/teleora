

extern crate permutohedron;

use permutohedron::Heap;


use super::{Statement, Frame, frame::empty};

pub struct PermutationIterator<'a, A, B> {
    list1: &'a [A],
    heap: Heap<'a, [B], B>
}

impl<'a, A, B> PermutationIterator<'a, A, B> {
    pub fn new(list1: &'a [A], list2: &'a mut [B]) -> Self {
        if list1.len() != list2.len() {
            panic!("PermuationIterator requires lists that are equal in length.");
        }
        return PermutationIterator {
            list1 : list1,
            heap : Heap::new(list2),
        };
    }
}

impl<'a, A, B> Iterator for PermutationIterator<'a, A, B> where A : Clone, B : Clone {
    type Item = (Vec<B>, Vec<A>);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(permutation) = self.heap.next_permutation() {
            return Some((permutation.to_vec(), self.list1.to_vec()))
        } else {
            None
        }
    }
}

pub struct PermutationFrameIterator {
    parent_frame : Box<Frame>,
    permutation_iter : PermutationIterator<'static, Statement, Statement>,
    frame_iter : Box<dyn Iterator<Item=Frame>>,
}

impl PermutationFrameIterator {

    pub fn new(list1 : & [Statement], list2 : & [Statement], frame : &Frame) -> Box<dyn Iterator<Item=Frame>> {
        let mut list2mut = list2.clone();

        let mut permutation_iter = PermutationIterator::new(list1, list2mut);



        let first_perm = permutation_iter.next();
        if let Some((fp1, fp2)) = first_perm {

        } else {
            return empty(); 
        }


        //let frame_iter = Frame::evaluate_sequence(first)

        return Box::new(PermutationFrameIterator { parent_frame: Box::new(frame.clone()), 
                                          permutation_iter: permutation_iter,
                                          frame_iter : empty()});

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
