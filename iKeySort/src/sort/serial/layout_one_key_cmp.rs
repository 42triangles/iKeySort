use crate::sort::bin_layout::BinLayout;
use crate::sort::buffer::MaybeUninitInit;
use crate::sort::key::{CmpFn, KeyFn, SortKey};
use alloc::vec::Vec;
use core::mem::MaybeUninit;

impl<K: SortKey> BinLayout<K> {
    #[inline]
    pub(super) fn sort_by_one_key_then_by<T, F1, F2>(
        &self,
        slice: &mut [T],
        key: F1,
        compare: F2,
    ) where
        T: Copy,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>,
    {
        let mut buffer: Vec<MaybeUninit<T>> = Vec::with_capacity(slice.len());
        unsafe {
            buffer.set_len(slice.len());
        }
        self.sort_by_one_key_and_uninit_buffer_then_by(slice, &mut buffer, key, compare);
    }

    #[inline]
    pub(super) fn sort_by_one_key_and_uninit_buffer_then_by<T, F1, F2>(
        &self,
        src: &mut [T],
        buf: &mut [MaybeUninit<T>],
        key: F1,
        compare: F2,
    ) where
        T: Copy,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>,
    {
        debug_assert_eq!(src.len(), buf.len());

        let mapper = self.spread_with_uninit_buffer(src, buf, key);

        // by this time the buffer should be fully initialized
        let init_buffer = buf.assume_init_slice_mut();

        if self.bin_width_is_one() {
            // all elements inside bins have the same key1
            // continue sort elements by compare
            mapper.sort_chunks_by(init_buffer, src, compare, true);
        } else {
            // start ping pong
            // invert src and buffer
            mapper.sort_chunks_by_one_key_then_by(init_buffer, src, key, compare, true);
        }
    }

    #[inline]
    pub(super) fn sort_by_one_key_and_buffer_then_by<T, F1, F2>(
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
            mapper.sort_chunks_by(buf, src, compare, copy_to_src);
        } else {
            // continue ping pong
            // invert src and buf
            mapper.sort_chunks_by_one_key_then_by(buf, src, key, compare, !copy_to_src);
        }
    }
}
