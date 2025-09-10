use crate::sort::buffer::{CopyFromNotOverlap, CopyNotOverlapValue, DoubleRangeSlices};
use crate::sort::key::{CmpFn, KeyFn, SortKey};
use crate::sort::key_sort::BIN_SORT_MIN;
use crate::sort::mapper::Mapper;
use crate::sort::serial::slice_one_key_cmp::OneKeyBinSortCmpSerial;

impl Mapper {
    #[inline]
    pub(crate) fn sort_chunks_by_one_key_then_by<K, T, F1, F2>(
        &self,
        src: &mut [T],
        buf: &mut [T],
        key: F1,
        compare: F2,
        copy_to_src: bool,
    ) where
        K: SortKey,
        T: Copy,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>,
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
                    sub_slice.sort_unstable_by(|a, b| key(a).cmp(&key(b)).then(compare(a, b)));
                    if copy_to_src {
                        buf.copy_to_range_from_not_overlap(sub_slice, range);
                    }
                }
                _ => {
                    let (sub_slice, sub_buffer) = range.mut_slices(src, buf);
                    sub_slice.sort_by_one_key_and_buffer_then_by(
                        sub_buffer,
                        key,
                        compare,
                        copy_to_src,
                    );
                }
            }
        }
    }
}
