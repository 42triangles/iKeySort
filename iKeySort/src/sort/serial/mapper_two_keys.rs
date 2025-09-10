use crate::sort::buffer::{CopyFromNotOverlap, CopyNotOverlapValue, DoubleRangeSlices};
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mapper::Mapper;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;
use core::cmp::Ordering;
use crate::sort::key_sort::BIN_SORT_MIN;

impl Mapper {
    #[inline]
    pub(crate) fn sort_chunks_by_two_keys<K1, K2, T, F1, F2>(
        &self,
        src: &mut [T],
        buf: &mut [T],
        key1: F1,
        key2: F2,
        copy_to_src: bool,
    ) where
        T: Copy,
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
    {
        const TINY_SORT_MAX: usize = BIN_SORT_MIN;

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
                    // SAFETY: mapper ranges never overlap; (src, buf) are distinct buffers.
                    let sub_slice = unsafe { src.get_unchecked_mut(range.clone()) };
                    sub_slice.sort_unstable_by(|a, b| {
                        let ordering = key1(a).cmp(&key1(b));
                        if ordering == Ordering::Equal {
                            key2(a).cmp(&key2(b))
                        } else {
                            ordering
                        }
                    });
                    if copy_to_src {
                        buf.copy_to_range_from_not_overlap(sub_slice, range);
                    }
                }
                _ => {
                    let (sub_slice, sub_buffer) = range.mut_slices(src, buf);
                    sub_slice.sort_by_two_keys_and_buffer(sub_buffer, key1, key2, copy_to_src);
                }
            }
        }
    }
}
