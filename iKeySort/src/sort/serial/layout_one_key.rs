use alloc::vec::Vec;
use core::mem::MaybeUninit;
use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{KeyFn, SortKey};

impl<K: SortKey> BinLayout<K> {
    #[inline]
    pub fn sort_by_one_key<T: Copy, F: KeyFn<T, K>>(&self, slice: &mut [T], key: F) {
        let mut buffer: Vec<MaybeUninit<T>> = Vec::with_capacity(slice.len());
        unsafe { buffer.set_len(slice.len()); }
        self.sort_by_one_key_and_buffer(slice, &mut buffer, key);
    }
}

impl<K: SortKey> BinLayout<K> {
    #[inline]
    pub fn sort_by_one_key_and_buffer<T: Copy, F: KeyFn<T, K>>(
        &self,
        slice: &mut [T],
        buffer: &mut [MaybeUninit<T>],
        key: F,
    ) {
        debug_assert_eq!(slice.len(), buffer.len());

        let mapper = self.spread_with_buffer(slice, buffer, key);

        if !self.bin_width_is_one() {
            mapper.sort_chunks_by_one_key(slice, buffer, key);
        }
    }
}
