use crate::sort::bin_layout::BinLayout;
use crate::sort::buffer::{MaybeUninitInit, MaybeUninitResize};
use crate::sort::key::{CmpFn, KeyFn, SortKey};
use alloc::vec::Vec;
use core::mem::MaybeUninit;

impl<K: SortKey> BinLayout<K> {
    #[inline]
    pub(super) fn sort_by_one_key_then_by_and_uninit_buffer<T, F1, F2>(
        &self,
        src: &mut [T],
        buf: &mut Vec<MaybeUninit<T>>,
        key: F1,
        compare: F2,
    ) where
        T: Copy,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>,
    {
        buf.resize_to_new_len(src.len());

        let mapper = self.spread_with_uninit_buffer(src, buf, key);

        // by this time the buffer should be fully initialized
        let init_buffer = buf.assume_init_slice_mut();

        if self.bin_width_is_one() {
            // all elements inside bins have the same key1
            // continue sort elements by compare
            mapper.sort_chunks_by(src, init_buffer, compare, true);
        } else {
            mapper.sort_chunks_by_one_key_then_by(src, init_buffer, key, compare, true);
        }
    }

    #[inline]
    pub(super) fn sort_by_one_key_then_by_and_buffer<T, F1, F2>(
        &self,
        src: &mut [T],
        buf: &mut [T],
        key: F1,
        compare: F2,
        copy_to_src: bool,
    ) where
        T: Copy,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>,
    {
        debug_assert_eq!(src.len(), buf.len());

        let mapper = self.spread_with_buffer(src, buf, key);

        if self.bin_width_is_one() {
            // all elements inside bins have the same key1
            // continue sort elements by compare
            mapper.sort_chunks_by(src, buf, compare, copy_to_src);
        } else {
            mapper.sort_chunks_by_one_key_then_by(src, buf, key, compare, copy_to_src);
        }
    }
}
