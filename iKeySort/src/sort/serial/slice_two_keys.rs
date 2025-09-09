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
        buf: &mut [T],
        key1: F1,
        key2: F2,
        copy_back: bool,
    );

    fn sort_by_two_keys_and_uninit_buffer<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        buffer: &mut [MaybeUninit<T>],
        key1: F1,
        key2: F2,
    );
}

impl<T: Copy> TwoKeysBinSortSerial<T> for [T] {
    fn ser_sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        key1: F1,
        key2: F2,
    ) {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_two_keys(self, key1, key2);
        } else {
            // all bins already sorted by key1
            self.ser_sort_by_one_key(key2);
        };
    }

    #[inline]
    fn sort_by_two_keys_and_buffer<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        buffer: &mut [T],
        key1: F1,
        key2: F2,
        copy_back: bool,
    ) {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_two_keys_and_buffer(self, buffer, key1, key2, copy_back);
        } else {
            // already sorted by key1
            self.sort_by_one_key_and_buffer(buffer, key2, copy_back);
        }
    }

    #[inline]
    fn sort_by_two_keys_and_uninit_buffer<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        buf: &mut [MaybeUninit<T>],
        key1: F1,
        key2: F2,
    ) {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_two_keys_and_uninit_buffer(self, buf, key1, key2);
        } else {
            // all bins already sorted by key1
            self.sort_by_one_key_and_uninit_buffer(buf, key2);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;

    #[test]
    fn test_0() {
        let mut org: Vec<_> = reversed_2d_array(2);
        let mut arr = org.clone();
        arr.ser_sort_by_two_keys(|a| a.0, |a| a.1);
        org.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        assert_eq!(arr, org);
    }

    #[test]
    fn test_1() {
        let mut org: Vec<_> = reversed_2d_array(100_u64.isqrt() as usize);
        let mut arr = org.clone();
        arr.ser_sort_by_two_keys(|a| a.0, |a| a.1);
        org.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        assert_eq!(arr, org);
    }

    #[test]
    fn test_2() {
        let mut org: Vec<_> = reversed_2d_array(1000_u64.isqrt() as usize);
        let mut arr = org.clone();
        arr.ser_sort_by_two_keys(|a| a.0, |a| a.1);
        org.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        assert_eq!(arr, org);
    }

    #[test]
    fn test_3() {
        let mut org: Vec<_> = reversed_2d_array(10_000_u64.isqrt() as usize);
        let mut arr = org.clone();
        arr.ser_sort_by_two_keys(|a| a.0, |a| a.1);
        org.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        assert_eq!(arr, org);
    }

    #[test]
    fn test_4() {
        let mut org: Vec<_> = reversed_2d_array(100_000_u64.isqrt() as usize);
        let mut arr = org.clone();
        arr.ser_sort_by_two_keys(|a| a.0, |a| a.1);
        org.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        assert_eq!(arr, org);
    }

    #[test]
    fn test_5() {
        let mut org: Vec<_> = reversed_2d_array(1000_000_u64.isqrt() as usize);
        let mut arr = org.clone();
        arr.ser_sort_by_two_keys(|a| a.0, |a| a.1);
        org.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        assert_eq!(arr, org);
    }

    fn reversed_2d_array(count: usize) -> Vec<(i32, i32)> {
        let mut arr = Vec::with_capacity(count * count);
        for x in (0..count as i32).rev() {
            for y in (0..count as i32).rev() {
                arr.push((x, y))
            }
        }

        arr
    }
}
