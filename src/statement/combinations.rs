use std::fmt;
use std::iter::FusedIterator;
use alloc::vec::Vec;
use crate::complement::ComplementIndices;

// TODO we can get rid of lazy_buffer, its actually less efficient here because we are always taking all elements from the iterator!

macro_rules! debug_fmt_fields {
    ($tyname:ident, $($($field:tt/*TODO ideally we would accept ident or tuple element here*/).+),*) => {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.debug_struct(stringify!($tyname))
                $(
              .field(stringify!($($field).+), &self.$($field).+)
              )*
              .finish()
        }
    }
}

macro_rules! clone_fields {
    ($($field:ident),*) => {
        #[inline] // TODO is this sensible?
        fn clone(&self) -> Self {
            Self {
                $($field: self.$field.clone(),)*
            }
        }
    }
}

macro_rules! ignore_ident{
    ($id:ident, $($t:tt)*) => {$($t)*};
}


#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Combinations<I: Iterator> {
    indices: Vec<usize>,
    pool: Vec<I::Item>,
    first: bool,
    n : usize,
}

impl<I> Clone for Combinations<I>
    where I: Clone + Iterator,
          I::Item: Clone,
{
    clone_fields!(indices, pool, first, n);
}

impl<I> fmt::Debug for Combinations<I>
    where I: Iterator + fmt::Debug,
          I::Item: fmt::Debug,
{
    debug_fmt_fields!(Combinations, indices, pool, first);
}

/// Create a new `Combinations` from a clonable iterator.
pub fn combinations<I>(iter: I, k : usize) -> Combinations<I>
    where I: Iterator
{
    let mut pool : Vec<_> = iter.collect();
    let n = pool.len();
    Combinations {
        indices: (0..k).collect(), 
        pool : pool,
        first: true,
        n : n,
    }
}

impl<I: Iterator> Combinations<I> {
    /// Returns the length of a combination produced by this iterator.
    #[inline]
    pub fn k(&self) -> usize { self.indices.len() }

    /// Resets this `Combinations` back to an initial state for combinations of length
    /// `k` over the same pool data source.
    pub(crate) fn reset(&mut self, k: usize) {
        self.indices = (0..k).collect();
    }
}


impl<I> Iterator for Combinations<I>
    where I: Iterator,
          I::Item: Clone
{
    type Item = (Vec<I::Item>, Vec<I::Item>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            if self.k() > self.n {
                return None;
            }
            self.first = false;
        } else if self.indices.is_empty() {
            return None;
        } else {
            // Scan from the end, looking for an index to increment
            let mut i: usize = self.indices.len() - 1;

            while self.indices[i] == i + self.pool.len() - self.indices.len() {
                if i > 0 {
                    i -= 1;
                } else {
                    // Reached the last combination
                    return None;
                }
            }

            // Increment index, and reset the ones to its right
            self.indices[i] += 1;
            for j in i+1..self.indices.len() {
                self.indices[j] = self.indices[j - 1] + 1;
            }
        }
        // println!("--{:?}", self.indices);
        // Create result vector based on the indices
        let p = self.indices.iter().map(|i| self.pool[*i].clone()).collect();
        let n = ComplementIndices::new(&self.indices, self.n).map(|i| self.pool[i].clone()).collect();
        return Some((p, n));
    }
}

impl<I> FusedIterator for Combinations<I>
    where I: Iterator,
          I::Item: Clone
{}

impl<I: Iterator> CombinationSplit for I {}

pub trait CombinationSplit : Iterator {

    fn combinations_split(self, k: usize) -> Combinations<Self>
        where Self: Sized,
              Self::Item: Clone{
        return combinations(self, k);
    }
}
