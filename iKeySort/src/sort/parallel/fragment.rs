use std::mem::MaybeUninit;
use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mapper::Mapper;
use std::ops::Range;

pub(super) struct Fragment<'a, T> {
    pub(super) base: usize,
    pub(super) src: &'a mut [T],
    pub(super) buf: &'a mut [MaybeUninit<T>],
}

pub(super) struct IdRange {
    pub(super) index: usize,
    pub(super) range: Range<usize>
}

impl<T> Fragment<'_, T>
where
    T: Send + Copy,
{
    #[inline]
    pub(super) fn spread<K, F>(&mut self, layout: BinLayout<K>, key: F) -> Vec<IdRange>
    where
        K: SortKey,
        F: KeyFn<T, K>,
    {
        let mut mapper = Mapper::new(layout.count());
        for a in self.src.iter() {
            mapper.inc_bin_count(layout.index(key(a)));
        }

        mapper.init_indices();

        for val in self.src.iter() {
            let index = mapper.next_index(layout.index(key(val)));
            unsafe {
                self.buf.get_unchecked_mut(index).write(*val);
            }
        }

        let mut ranges = Vec::with_capacity(mapper.count);
        for (index, chunk) in mapper.iter().enumerate() {
            let range = chunk.as_range();
            if !range.is_empty() {
                let global_range = range.start + self.base..range.end + self.base;
                ranges.push(IdRange {
                    index,
                    range: global_range,
                });
            }
        }

        ranges
    }
}
