use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;
use core::mem::MaybeUninit;

pub(crate) trait TwoKeysBinSortSerial<T> {
    fn ser_sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        key1: F1,
        key2: F2,
    );

    fn sort_by_two_keys_and_buffer<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        buffer: &mut [MaybeUninit<T>],
        key1: F1,
        key2: F2,
    );
}

impl<T: Copy> TwoKeysBinSortSerial<T> for [T] {
    #[inline]
    fn ser_sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        key1: F1,
        key2: F2,
    ) {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_two_keys(self, key1, key2);
        } else {
            // already sorted by key1
            self.ser_sort_by_one_key(key2);
        };
    }

    #[inline]
    fn sort_by_two_keys_and_buffer<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        buffer: &mut [MaybeUninit<T>],
        key1: F1,
        key2: F2,
    ) {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_two_keys_and_buffer(self, buffer, key1, key2);
        } else {
            // already sorted by key1
            self.sort_by_one_key_and_buffer(buffer, key2);
        }
    }
}
