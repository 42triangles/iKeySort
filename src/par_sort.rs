use std::cmp::Ordering;
use rayon::prelude::*;
use std::marker::PhantomData;
use crate::index::{BinKey, Offset};
use crate::key_sort::{Bin, KeyBinSort};

pub trait ParBinSort<U> {
    fn par_bin_sort_by<F>(&mut self, compare: F)
    where
        F: Fn(&Self::Item, &Self::Item) -> Ordering + Sync;

    type Item: Send;
}

impl<T, U> ParBinSort<U> for [T]
where
    T: BinKey<U> + Clone + Send,
    U: Copy + Ord + Offset,
{
    fn par_bin_sort_by<F>(&mut self, compare: F)
    where
        F: Fn(&T, &T) -> Ordering + Sync,
    {
        let bins = self.sort_by_bins();
        let mut_chunks = ChunksMutByBin::new(self, bins).par_bridge();
        mut_chunks.for_each(|slice|slice.sort_by(&compare));
    }

    type Item = T;
}

struct ChunksMutByBin<'a, T: 'a> {
    data: *mut [T],
    bins: Vec<Bin>,
    index: usize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T: 'a> ChunksMutByBin<'a, T> {
    #[inline]
    fn new(data: &'a mut [T], bins: Vec<Bin>) -> Self {
        Self { data, bins, index: 0, _marker: PhantomData }
    }
}

unsafe impl<'a, T: Send> Send for ChunksMutByBin<'a, T> {}

impl<'a, T> Iterator for ChunksMutByBin<'a, T> {
    type Item = &'a mut [T];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.bins.len() { return None };
        let capacity = self.data.len() >> 6;
        let start = self.bins[self.index].offset;
        while self.index < self.bins.len() {
            let end = self.bins[self.index].data;
            self.index += 1;
            let count = end - start;
            if count > capacity {
                let slice = unsafe { &mut *self.data }[start..end].as_mut();
                return Some(slice)
            }
        }

        if start < self.data.len() {
            let slice = unsafe { &mut *self.data }[start..].as_mut();
            Some(slice)
        } else {
            None
        }
    }
}