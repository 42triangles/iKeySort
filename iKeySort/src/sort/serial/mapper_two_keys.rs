use crate::sort::buffer::DoubleRangeSlices;
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mapper::Mapper;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;
use core::cmp::Ordering;

impl Mapper {
    #[inline]
    pub(crate) fn sort_chunks_by_two_keys<K: SortKey, T: Copy, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &self,
        src: &mut [T],
        buf: &mut [T],
        key1: F1,
        key2: F2,
        copy_to_src: bool,
    ) {
        const TINY_SORT_MAX: usize = 64;

        // if `copy_to_src` is true
        // must copy `src` to `buf`, since the result array is in the buffer

        for chunk in self.iter() {
            let range = chunk.as_range();
            match range.len() {
                0..=1 => {
                    if copy_to_src {
                        unsafe {
                            let dst = buf.get_unchecked_mut(range.start);
                            let val = src.get_unchecked(range.start);
                            *dst = *val;
                        }
                    }
                }
                2..TINY_SORT_MAX => {
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
                        let sub_buffer = unsafe { buf.get_unchecked_mut(range) };
                        sub_buffer.copy_from_slice(sub_slice);
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
