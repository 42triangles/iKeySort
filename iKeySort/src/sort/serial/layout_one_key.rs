use crate::sort::bin_layout::BinLayout;
use crate::sort::buffer::{CopyFromNotOverlap, MaybeUninitInit};
use crate::sort::key::{KeyFn, SortKey};
use alloc::vec::Vec;
use core::mem::MaybeUninit;

impl<K: SortKey> BinLayout<K> {
    #[inline]
    pub(super) fn sort_by_one_key<T: Copy, F: KeyFn<T, K>>(&self, slice: &mut [T], key: F) {
        let mut buffer: Vec<MaybeUninit<T>> = Vec::with_capacity(slice.len());
        unsafe {
            buffer.set_len(slice.len());
        }
        self.sort_by_one_key_and_uninit_buffer(slice, &mut buffer, key);
    }

    #[inline]
    pub(super) fn sort_by_one_key_and_uninit_buffer<T: Copy, F: KeyFn<T, K>>(
        &self,
        src: &mut [T],
        buf: &mut [MaybeUninit<T>],
        key: F,
    ) {
        debug_assert_eq!(src.len(), buf.len());

        let mapper = self.spread_with_uninit_buffer(src, buf, key);

        // by this time the buffer should be fully initialized
        let init_buffer = buf.assume_init_slice_mut();

        if self.bin_width_is_one() {
            // all elements inside bins have the same key
            // sort is finished
            // move all data from buffer to src
            src.copy_from_not_overlap(init_buffer);
        } else {
            // start ping pong
            // invert src and buffer
            mapper.sort_chunks_by_one_key(init_buffer, src, key, true);
        }
    }

    #[inline]
    pub(super) fn sort_by_one_key_and_buffer<T: Copy, F: KeyFn<T, K>>(
        &self,
        src: &mut [T],
        buf: &mut [T],
        key: F,
        copy_to_src: bool,
    ) {
        debug_assert_eq!(src.len(), buf.len());

        let mapper = self.spread_with_buffer(src, buf, key);

        if self.bin_width_is_one() {
            // all elements inside bins have the same key
            // sort is finished
            if copy_to_src {
                src.copy_from_not_overlap(buf);
            }
        } else {
            // continue ping pong
            // invert src and buf
            mapper.sort_chunks_by_one_key(buf, src, key, !copy_to_src);
        }
    }
}
