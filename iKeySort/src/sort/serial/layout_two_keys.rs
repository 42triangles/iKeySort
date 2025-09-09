use crate::sort::bin_layout::BinLayout;
use crate::sort::buffer::MaybeUninitInit;
use crate::sort::key::{KeyFn, SortKey};
use alloc::vec::Vec;
use core::mem::MaybeUninit;

impl<K: SortKey> BinLayout<K> {
    #[inline]
    pub(super) fn sort_by_two_keys<T: Copy, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &self,
        src: &mut [T],
        key1: F1,
        key2: F2,
    ) {
        let mut buf: Vec<MaybeUninit<T>> = Vec::with_capacity(src.len());
        unsafe {
            buf.set_len(src.len());
        }
        self.sort_by_two_keys_and_uninit_buffer(src, &mut buf, key1, key2);
    }

    #[inline]
    pub(super) fn sort_by_two_keys_and_uninit_buffer<T: Copy, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &self,
        src: &mut [T],
        buf: &mut [MaybeUninit<T>],
        key1: F1,
        key2: F2,
    ) {
        debug_assert_eq!(src.len(), buf.len());

        let mapper = self.spread_with_uninit_buffer(src, buf, key1);

        // by this time the buffer should be fully initialized
        let init_buffer = buf.assume_init_slice_mut();

        // start ping pong
        // invert src and buffer
        if self.bin_width_is_one() {
            // all elements inside bins have the same key1
            // continue sort elements inside bins by key2
            mapper.sort_chunks_by_one_key(init_buffer, src, key2, true);
        } else {
            mapper.sort_chunks_by_two_keys(init_buffer, src, key1, key2, true);
        }
    }

    #[inline]
    pub(super) fn sort_by_two_keys_and_buffer<T: Copy, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &self,
        src: &mut [T],
        buf: &mut [T],
        key1: F1,
        key2: F2,
        copy_back: bool,
    ) {
        debug_assert_eq!(src.len(), buf.len());

        let mapper = self.spread_with_buffer(src, buf, key1);

        // continue ping pong
        // invert src and buf

        if self.bin_width_is_one() {
            // all elements inside bins have the same key1
            // continue sort elements inside bins by key2
            mapper.sort_chunks_by_one_key(buf, src, key2, !copy_back);
        } else {
            mapper.sort_chunks_by_two_keys(buf, src, key1, key2, !copy_back);
        }
    }
}
