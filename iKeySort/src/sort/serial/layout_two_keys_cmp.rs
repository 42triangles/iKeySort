use crate::sort::bin_layout::BinLayout;
use crate::sort::buffer::MaybeUninitInit;
use crate::sort::key::{CmpFn, KeyFn, SortKey};
use alloc::vec::Vec;
use core::mem::MaybeUninit;

impl<K: SortKey> BinLayout<K> {
    #[inline]
    pub(super) fn sort_by_two_keys_then_by<T, F1, F2, F3>(
        &self,
        src: &mut [T],
        key1: F1,
        key2: F2,
        compare: F3,
    ) where
        T: Copy,
        F1: KeyFn<T, K>,
        F2: KeyFn<T, K>,
        F3: CmpFn<T>,
    {
        let mut buf: Vec<MaybeUninit<T>> = Vec::with_capacity(src.len());
        unsafe {
            buf.set_len(src.len());
        }
        self.sort_by_two_keys_and_uninit_buffer_then_by(src, &mut buf, key1, key2, compare);
    }

    #[inline]
    pub(super) fn sort_by_two_keys_and_uninit_buffer_then_by<T, F1, F2, F3>(
        &self,
        src: &mut [T],
        buf: &mut [MaybeUninit<T>],
        key1: F1,
        key2: F2,
        compare: F3,
    ) where
        T: Copy,
        F1: KeyFn<T, K>,
        F2: KeyFn<T, K>,
        F3: CmpFn<T>,
    {
        debug_assert_eq!(src.len(), buf.len());

        let mapper = self.spread_with_uninit_buffer(src, buf, key1);

        // by this time the buffer should be fully initialized
        let init_buffer = buf.assume_init_slice_mut();

        // start ping pong
        // invert src and buffer
        if self.bin_width_is_one() {
            // all elements inside bins have the same key1
            // continue sort elements inside bins by key2
            mapper.sort_chunks_by_one_key_then_by(init_buffer, src, key2, compare,true);
        } else {
            mapper.sort_chunks_by_two_keys_then_by(init_buffer, src, key1, key2, compare, true);
        }
    }

    #[inline]
    pub(super) fn sort_by_two_keys_and_buffer_then_by<T, F1, F2, F3>(
        &self,
        src: &mut [T],
        buf: &mut [T],
        key1: F1,
        key2: F2,
        compare: F3,
        copy_to_src: bool,
    ) where
        T: Copy,
        F1: KeyFn<T, K>,
        F2: KeyFn<T, K>,
        F3: CmpFn<T>
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
