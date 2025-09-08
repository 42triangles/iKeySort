use alloc::vec::Vec;
use core::mem::MaybeUninit;
use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{KeyFn, SortKey};

impl<K: SortKey> BinLayout<K> {
    #[inline]
    pub fn sort_by_two_keys<T: Copy, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &self,
        slice: &mut [T],
        key1: F1,
        key2: F2,
    ) {
        let mut buffer: Vec<MaybeUninit<T>> = Vec::with_capacity(slice.len());
        unsafe { buffer.set_len(slice.len()); }
        self.sort_by_two_keys_and_buffer(slice, &mut buffer, key1, key2);
    }

    #[inline]
    pub fn sort_by_two_keys_and_buffer<T: Copy, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &self,
        slice: &mut [T],
        buffer: &mut [MaybeUninit<T>],
        key1: F1,
        key2: F2,
    ) {
        debug_assert_eq!(slice.len(), buffer.len());

        let mapper = self.spread_with_buffer(slice, buffer, key1);

        if self.bin_width_is_one() {
            mapper.sort_chunks_by_one_key(slice, buffer, key2);
        } else {
            mapper.sort_chunks_by_two_keys(slice, buffer, key1, key2);
        }
    }
}
