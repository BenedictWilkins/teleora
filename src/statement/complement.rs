

pub fn complement_indices(indices: &[usize], n: usize) -> Vec<usize> {
    let mut complement = Vec::with_capacity(n - indices.len());
    let mut iter = indices.iter().copied().peekable();
    let mut next_index = 0;
    while next_index < n {
        if iter.peek() == Some(&next_index) {
            iter.next();
        } else {
            complement.push(next_index);
        }
        next_index += 1;
    }
    return complement
}

pub struct ComplementIndices<'a> {
    iter: std::iter::Peekable<std::iter::Copied<std::slice::Iter<'a, usize>>>,
    next_index: usize,
    n: usize,
}

impl<'a> ComplementIndices<'a> {
    pub fn new(indices: &'a [usize], n: usize) -> ComplementIndices<'a> {
        let iter = indices.iter().copied().peekable();
        ComplementIndices {
            iter,
            next_index: 0,
            n,
        }
    }
}

impl<'a> Iterator for ComplementIndices<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next_index < self.n {
            if Some(&self.next_index) == self.iter.peek() {
                self.iter.next();
            } else {
                let complement_index = self.next_index;
                self.next_index += 1;
                return Some(complement_index);
            }
            self.next_index += 1;
        }
        None
    }
}

