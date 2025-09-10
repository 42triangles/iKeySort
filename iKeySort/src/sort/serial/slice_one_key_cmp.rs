use crate::sort::bin_layout::BinLayout;
use crate::sort::buffer::CopyFromNotOverlap;
use crate::sort::key::{CmpFn, KeyFn, SortKey};
use core::mem::MaybeUninit;

pub(crate) trait OneKeyBinSortCmpSerial<T> {
    fn ser_sort_by_one_key_then_by<K, F1, F2>(&mut self, key1: F1, compare: F2)
    where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>;

    fn sort_by_one_key_and_buffer_then_by<K, F1, F2>(
        &mut self,
        buf: &mut [T],
        key1: F1,
        compare: F2,
        copy_to_src: bool,
    ) where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>;

    fn sort_by_one_key_and_uninit_buffer_then_by<K, F1, F2>(
        &mut self,
        buffer: &mut [MaybeUninit<T>],
        key1: F1,
        compare: F2,
    ) where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>;
}

impl<T: Copy> OneKeyBinSortCmpSerial<T> for [T] {
    fn ser_sort_by_one_key_then_by<K, F1, F2>(&mut self, key1: F1, compare: F2)
    where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>,
    {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_one_key_then_by(self, key1, compare);
        } else {
            // one bin with single key for all elements
            self.sort_unstable_by(compare);
        };
    }

    #[inline]
    fn sort_by_one_key_and_buffer_then_by<K, F1, F2>(
        &mut self,
        buffer: &mut [T],
        key1: F1,
        compare: F2,
        copy_to_src: bool,
    ) where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>,
    {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_one_key_and_buffer_then_by(self, buffer, key1, compare, copy_to_src);
        } else {
            // one bin with single key for all elements
            self.sort_unstable_by(compare);
            if copy_to_src {
                buffer.copy_from_not_overlap(self);
            }
        }
    }

    #[inline]
    fn sort_by_one_key_and_uninit_buffer_then_by<K, F1, F2>(
        &mut self,
        buf: &mut [MaybeUninit<T>],
        key1: F1,
        compare: F2,
    ) where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>,
    {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_one_key_and_uninit_buffer_then_by(self, buf, key1, compare);
        } else {
            // one bin with single key for all elements
            self.sort_unstable_by(compare);
        }
    }
}
