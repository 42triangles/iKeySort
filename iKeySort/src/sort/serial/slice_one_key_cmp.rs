use crate::sort::bin_layout::BinLayout;
use crate::sort::buffer::CopyFromNotOverlap;
use crate::sort::key::{CmpFn, KeyFn, SortKey};

#[cfg(feature = "allow_multithreading")]
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

    #[cfg(feature = "allow_multithreading")]
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

    #[cfg(feature = "allow_multithreading")]
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

#[cfg(test)]
mod tests {
    use crate::sort::serial::slice_one_key_cmp::OneKeyBinSortCmpSerial;
    use alloc::vec::Vec;

    #[test]
    fn test_0() {
        test(2);
    }

    #[test]
    fn test_1() {
        test(10);
    }

    #[test]
    fn test_2() {
        test(30);
    }

    #[test]
    fn test_3() {
        test(100);
    }

    #[test]
    fn test_4() {
        test(300);
    }

    #[test]
    fn test_5() {
        test(1000);
    }

    fn test(count: usize) {
        let mut org: Vec<_> = reversed_2d_array(count);
        let mut arr = org.clone();
        arr.ser_sort_by_one_key_then_by(|a| a.0, |a, b| a.1.cmp(&b.1));
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
