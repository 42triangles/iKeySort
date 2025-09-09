use core::mem::MaybeUninit;
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mapper::Mapper;
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;

impl Mapper {
    #[inline]
    pub(crate) fn sort_chunks_by_one_key<K: SortKey, T: Copy, F: KeyFn<T, K>>(
        &self,
        slice: &mut [T],
        buffer: &mut [MaybeUninit<T>],
        key: F,
    ) {
        for chunk in self.iter() {
            match chunk.count() {
                0..=1 => {
                    continue;
                }
                2..=63 => {
                    let sub_slice = unsafe { slice.get_unchecked_mut(chunk.as_range()) };
                    sub_slice.sort_unstable_by_key(|val| key(val));
                }
                _ => {
                    let range = chunk.as_range();
                    let (sub_slice, sub_buffer) = unsafe {
                        let sub_buffer = buffer.get_unchecked_mut(..range.len());
                        let sub_slice = slice.get_unchecked_mut(range);
                        (sub_slice, sub_buffer)
                    };

                    sub_slice.sort_by_one_key_and_buffer(sub_buffer, key);
                }
            }
        }
    }
}