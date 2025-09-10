use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{KeyFn, SortKey};
use core::mem::MaybeUninit;

pub(crate) trait OneKeyBinSortSerial<T> {
    fn ser_sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, key: F);

    fn sort_by_one_key_and_buffer<K: SortKey, F: KeyFn<T, K>>(
        &mut self,
        buf: &mut [T],
        key: F,
        copy_to_src: bool,
    );

    fn sort_by_one_key_and_uninit_buffer<K: SortKey, F: KeyFn<T, K>>(
        &mut self,
        buf: &mut [MaybeUninit<T>],
        key: F,
    );
}

impl<T: Copy> OneKeyBinSortSerial<T> for [T] {
    #[inline]
    fn ser_sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, key: F) {
        if let Some(layout) = BinLayout::with_keys(self, key) {
            layout.sort_by_one_key(self, key);
        }
    }

    #[inline]
    fn sort_by_one_key_and_buffer<K: SortKey, F: KeyFn<T, K>>(
        &mut self,
        buf: &mut [T],
        key: F,
        copy_to_src: bool,
    ) {
        if let Some(layout) = BinLayout::with_keys(self, key) {
            layout.sort_by_one_key_and_buffer(self, buf, key, copy_to_src);
        }
    }

    #[inline]
    fn sort_by_one_key_and_uninit_buffer<K: SortKey, F: KeyFn<T, K>>(
        &mut self,
        buf: &mut [MaybeUninit<T>],
        key: F,
    ) {
        if let Some(layout) = BinLayout::with_keys(self, key) {
            layout.sort_by_one_key_and_uninit_buffer(self, buf, key);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;

    #[test]
    fn test_0() {
        test(10);
    }

    #[test]
    fn test_1() {
        test(100);
    }

    #[test]
    fn test_2() {
        test(1_000);
    }

    #[test]
    fn test_3() {
        test(10_000);
    }

    #[test]
    fn test_4() {
        test(100_000);
    }

    #[test]
    fn test_5() {
        test(1000_000);
    }

    fn test(count: usize) {
        let mut org: Vec<_> = (0..count).rev().collect();
        let mut arr = org.clone();
        arr.ser_sort_by_one_key(|&a| a);
        org.sort_unstable();
        assert_eq!(arr, org);
    }
}
