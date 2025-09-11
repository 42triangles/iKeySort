use crate::sort::bin_layout::BinLayout;
use crate::sort::buffer::{MaybeUninitInit, MaybeUninitResize};
use crate::sort::key::{CmpFn, KeyFn, SortKey};
use alloc::vec::Vec;
use core::mem::MaybeUninit;

impl<K1: SortKey> BinLayout<K1> {
    #[inline]
    pub(super) fn sort_by_two_keys_then_by_and_uninit_buffer<T, K2, F1, F2, F3>(
        &self,
        src: &mut [T],
        buf: &mut Vec<MaybeUninit<T>>,
        key1: F1,
        key2: F2,
        compare: F3,
    ) where
        T: Copy,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
        F3: CmpFn<T>,
    {
        buf.resize_to_new_len(src.len());

        let mapper = self.spread_with_uninit_buffer(src, buf, key1);

        // by this time the buffer should be fully initialized
        let init_buffer = buf.assume_init_slice_mut();

        // start ping pong
        // invert src and buffer
        if self.bin_width_is_one() {
            // all elements inside bins have the same key1
            // continue sort elements inside bins by key2
            mapper.sort_chunks_by_one_key_then_by(init_buffer, src, key2, compare, true);
        } else {
            mapper.sort_chunks_by_two_keys_then_by(init_buffer, src, key1, key2, compare, true);
        }
    }

    #[inline]
    pub(super) fn sort_by_two_keys_then_by_and_buffer<T, K2, F1, F2, F3>(
        &self,
        src: &mut [T],
        buf: &mut [T],
        key1: F1,
        key2: F2,
        compare: F3,
        copy_to_src: bool,
    ) where
        T: Copy,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
        F3: CmpFn<T>,
    {
        debug_assert_eq!(src.len(), buf.len());

        let mapper = self.spread_with_buffer(src, buf, key1);

        // continue ping pong
        // invert src and buf

        if self.bin_width_is_one() {
            // all elements inside bins have the same key1
            // continue sort elements inside bins by key2
            mapper.sort_chunks_by_one_key_then_by(buf, src, key2, compare, !copy_to_src);
        } else {
            mapper.sort_chunks_by_two_keys_then_by(buf, src, key1, key2, compare, !copy_to_src);
        }
    }
}
