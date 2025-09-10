use crate::sort::buffer::{CopyFromNotOverlap, CopyNotOverlapValue, DoubleRangeSlices};
use crate::sort::key::{CmpFn, KeyFn, SortKey};
use crate::sort::mapper::Mapper;
use crate::sort::serial::slice_two_keys_cmp::TwoKeysBinSortCmpSerial;

impl Mapper {
    #[inline]
    pub(crate) fn sort_chunks_by_two_keys_then_by<K, T, F1, F2, F3>(
        &self,
        src: &mut [T],
        buf: &mut [T],
        key1: F1,
        key2: F2,
        compare: F3,
        copy_to_src: bool,
    ) where
        K: SortKey,
        T: Copy,
        F1: KeyFn<T, K>,
        F2: KeyFn<T, K>,
        F3: CmpFn<T>,
    {
        const TINY_SORT_MAX: usize = 64;

        // if `copy_to_src` is true
        // must copy `src` to `buf`, since the result array is in the buffer

        for chunk in self.iter() {
            let range = chunk.as_range();
            match range.len() {
                0 => continue,
                1 => {
                    if copy_to_src {
                        buf.copy_value_from(src, range.start);
                    }
                }
                2..TINY_SORT_MAX => {
                    let sub_slice = unsafe { src.get_unchecked_mut(range.clone()) };
                    sub_slice.sort_unstable_by(|a, b| {
                        key1(a)
                            .cmp(&key1(b))
                            .then(key2(a).cmp(&key2(b)))
                            .then(compare(a, b))
                    });
                    if copy_to_src {
                        buf.copy_to_range_from_not_overlap(sub_slice, range);
                    }
                }
                _ => {
                    let (sub_slice, sub_buffer) = range.mut_slices(src, buf);
                    sub_slice.sort_by_two_keys_and_buffer_then_by(
                        sub_buffer,
                        key1,
                        key2,
                        compare,
                        copy_to_src,
                    );
                }
            }
        }
    }
}
