use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{CmpFn, KeyFn, SortKey};
use crate::sort::serial::slice_one_key_cmp::OneKeyBinSortCmpSerial;
use core::mem::MaybeUninit;

pub(crate) trait TwoKeysBinSortCmpSerial<T> {
    fn ser_sort_by_two_keys_then_by<K1, K2, F1, F2, F3>(&mut self, key1: F1, key2: F2, compare: F3)
    where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
        F3: CmpFn<T>;

    fn sort_by_two_keys_and_buffer_then_by<K1, K2, F1, F2, F3>(
        &mut self,
        buf: &mut [T],
        key1: F1,
        key2: F2,
        compare: F3,
        copy_to_src: bool,
    ) where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
        F3: CmpFn<T>;

    fn sort_by_two_keys_and_uninit_buffer_then_by<K1, K2, F1, F2, F3>(
        &mut self,
        buffer: &mut [MaybeUninit<T>],
        key1: F1,
        key2: F2,
        compare: F3,
    ) where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
        F3: CmpFn<T>;
}

impl<T: Copy> TwoKeysBinSortCmpSerial<T> for [T] {
    fn ser_sort_by_two_keys_then_by<K1, K2, F1, F2, F3>(&mut self, key1: F1, key2: F2, compare: F3)
    where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
        F3: CmpFn<T>,
    {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_two_keys_then_by(self, key1, key2, compare);
        } else {
            // all bins already sorted by key1
            self.ser_sort_by_one_key_then_by(key2, compare);
        };
    }

    #[inline]
    fn sort_by_two_keys_and_buffer_then_by<K1, K2, F1, F2, F3>(
        &mut self,
        buffer: &mut [T],
        key1: F1,
        key2: F2,
        compare: F3,
        copy_to_src: bool,
    ) where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
        F3: CmpFn<T>,
    {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_two_keys_and_buffer_then_by(
                self,
                buffer,
                key1,
                key2,
                compare,
                copy_to_src,
            );
        } else {
            // already sorted by key1
            self.sort_by_one_key_and_buffer_then_by(buffer, key2, compare, copy_to_src);
        }
    }

    #[inline]
    fn sort_by_two_keys_and_uninit_buffer_then_by<K1, K2, F1, F2, F3>(
        &mut self,
        buf: &mut [MaybeUninit<T>],
        key1: F1,
        key2: F2,
        compare: F3,
    ) where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
        F3: CmpFn<T>,
    {
        if let Some(layout) = BinLayout::with_keys(self, key1) {
            layout.sort_by_two_keys_and_uninit_buffer_then_by(self, buf, key1, key2, compare);
        } else {
            // all bins already sorted by key1
            self.sort_by_one_key_and_uninit_buffer_then_by(buf, key2, compare);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;
    use crate::sort::serial::slice_two_keys_cmp::TwoKeysBinSortCmpSerial;

    #[test]
    fn test_0() {
        let mut org: Vec<_> = reversed_2d_array(2);
        let mut arr = org.clone();
        arr.ser_sort_by_two_keys_then_by(|a| a.0, |a| a.1, |a, b| a.2.cmp(&b.2));
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

    fn reversed_2d_array(count: usize) -> Vec<(u32, i32, i32)> {
        let mut arr = Vec::with_capacity(count * count * count);
        for i in (0..count as u32).rev() {
            for x in (0..count as i32).rev() {
                for y in (0..count as i32).rev() {
                    arr.push((i, x, y))
                }
            }
        }

        arr
    }
}
