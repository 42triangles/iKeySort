use std::cmp::Ordering;
use crate::index::{BinKey, BinLayout, BinLayoutOp};
use crate::key_sort::Bin;

pub struct BinStore<U> {
    pub layout: BinLayout<U>,
    pub bins: Vec<Bin>
}

impl<U: Copy + Ord + BinLayoutOp> BinStore<U> {
    #[inline]
    pub fn new(min: U, max: U, count: usize) -> Option<Self> {
        let layout = BinLayout::new(min..max, count)?;
        let bin_count = layout.index(max) + 1;
        let bins = vec![Bin { offset: 0, data: 0 }; bin_count];

        Some(Self {
            layout,
            bins
        })
    }

    #[inline]
    pub fn layout_bins<'a, I, T>(&mut self, iter: I)
    where
        I: Iterator<Item = &'a T>,
        T: 'a + BinKey<U> + Clone,
    {
        // calculate capacity for each bin
        for p in iter {
            let index = p.bin_index(&self.layout);
            unsafe { self.bins.get_unchecked_mut(index) }.data += 1;
        }

        // calculate range for each bin
        let mut offset = 0;
        for bin in self.bins.iter_mut() {
            let next_offset = offset + bin.data;
            bin.offset = offset; // offset from start
            bin.data = offset; // iterator cursor
            offset = next_offset;
        }
    }

    #[inline]
    pub fn into_sorted_by_bins_vec<I, T, F>(self, count: usize, iter: I, compare: F) -> Vec<T>
    where
        I: IntoIterator<Item = T>,
        T: BinKey<U> + Clone + Default,
        F: FnMut(&T, &T) -> Ordering
    {
        let layout = self.layout;
        let mut bins = self.bins;
        let mut result = vec![T::default(); count];

        for p in iter {
            let index = p.bin_index(&layout);
            unsafe {
                let bin = bins.get_unchecked_mut(index);
                let item_index = bin.data;
                bin.data += 1;

                *result.get_unchecked_mut(item_index) = p;
            }
        }

        result.sort_by(compare);

        result
    }
}


#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::cmp::Ordering::Greater;
    use crate::layout::BinStore;
    use crate::min_max::min_max;

    #[test]
    fn test_0() {
        let array = vec![8, 3, 2, 6, 2, 1];
        let &min = array.iter().min().unwrap();
        let &max = array.iter().max().unwrap();
        let count = array.len();

        let mut store = BinStore::new(min, max, count).unwrap();
        store.layout_bins(array.iter());

        let bin_sorted = store.into_sorted_by_bins_vec(count, array.into_iter(), |a, b| a.cmp(b));

        assert_eq!(bin_sorted.len(), count);
        for w in bin_sorted.windows(2) {
            assert_ne!(w[0].cmp(&w[1]), Greater);
        }
    }

    #[test]
    fn test_random_bin_sort() {
        const COUNT: usize = 1000;
        let mut rng = rand::rng();

        for _ in 0..100 {
            let array: Vec<i32> = (0..COUNT).map(|_| rng.random_range(0..100)).collect();

            let (&min, &max) = min_max(array.iter()).unwrap();
            
            let mut store= if let Some(store) = BinStore::new(min, max, COUNT) {
                store
            } else {
                continue;
            };
            store.layout_bins(array.iter());

            let sorted = store.into_sorted_by_bins_vec(COUNT, array.into_iter(), |a, b| a.cmp(b));

            assert_eq!(sorted.len(), COUNT);

            for w in sorted.windows(2) {
                assert_ne!(w[0].cmp(&w[1]), Greater);
            }            
        }
    }
}
