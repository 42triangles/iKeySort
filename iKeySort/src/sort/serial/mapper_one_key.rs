use crate::sort::buffer::DoubleRangeSlices;
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mapper::Mapper;
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;

impl Mapper {
    #[inline]
    pub(crate) fn sort_chunks_by_one_key<K: SortKey, T: Copy, F: KeyFn<T, K>>(
        &self,
        src: &mut [T],
        buf: &mut [T],
        key: F,
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
                    continue;
                }
                2..TINY_SORT_MAX => {
                    let sub_slice = unsafe { src.get_unchecked_mut(range.clone()) };
                    sub_slice.sort_unstable_by_key(|val| key(val));
                    if copy_to_src {
                        let sub_buffer = unsafe { buf.get_unchecked_mut(range) };
                        sub_buffer.copy_from_slice(sub_slice);
                    }
                }
                _ => {
                    let (sub_slice, sub_buffer) = range.mut_slices(src, buf);
                    sub_slice.sort_by_one_key_and_buffer(sub_buffer, key, copy_to_src);
                }
            }
        }
    }
}
