use crate::sort::bin_layout::BinLayout;
use crate::sort::buffer::{CopyFromNotOverlap, MaybeUninitInit, MaybeUninitResize};
use crate::sort::key::{KeyFn, SortKey};
use alloc::vec::Vec;
use core::mem::MaybeUninit;

impl<K: SortKey> BinLayout<K> {
    #[inline]
    pub(super) fn sort_by_one_key_and_uninit_buffer<T: Copy, F: KeyFn<T, K>>(
        &self,
        src: &mut [T],
        buf: &mut Vec<MaybeUninit<T>>,
        key: F,
    ) {
        buf.resize_to_new_len(src.len());

        let mapper = self.spread_with_uninit_buffer(src, buf, key);

        // by this time the buffer should be fully initialized
        let init_buffer = buf.assume_init_slice_mut();

        if self.bin_width_is_one() {
            // all elements inside bins have the same key
            // sort is finished
            // move all data from buffer to src
            src.copy_from_not_overlap(init_buffer);
        } else {
            mapper.sort_chunks_by_one_key(src, init_buffer, key, true);
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
            mapper.sort_chunks_by_one_key(src, buf, key, copy_to_src);
        }
    }
}
