use crate::sort::bin_layout::BIN_SORT_MIN;
use crate::sort::key::{CmpFn, KeyFn, SortKey};
use crate::sort::serial::slice_two_keys_cmp::TwoKeysBinSortCmpSerial;
use core::mem::MaybeUninit;
use alloc::vec::Vec;

#[cfg(not(feature = "allow_multithreading"))]
pub trait TwoKeysAndCmpSort<T> {
    fn sort_by_two_keys_then_by<K1, K2, F1, F2, F3>(
        &mut self,
        parallel: bool,
        key1: F1,
        key2: F2,
        compare: F3,
    ) where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
        F3: CmpFn<T>;

    fn sort_by_two_keys_then_by_and_buffer<K1, K2, F1, F2, F3>(
        &mut self,
        parallel: bool,
        reusable_buffer: &mut Vec<MaybeUninit<T>>,
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

#[cfg(feature = "allow_multithreading")]
pub trait TwoKeysAndCmpSort<T> {
    fn sort_by_two_keys_then_by<K1, K2, F1, F2, F3>(
        &mut self,
        parallel: bool,
        key1: F1,
        key2: F2,
        compare: F3,
    ) where
        K1: SortKey + Send + Sync,
        K2: SortKey + Send + Sync,
        F1: KeyFn<T, K1> + Send + Sync,
        F2: KeyFn<T, K2> + Send + Sync,
        F3: CmpFn<T> + Send + Sync;

    fn sort_by_two_keys_then_by_and_buffer<K1, K2, F1, F2, F3>(
        &mut self,
        parallel: bool,
        reusable_buffer: &mut Vec<MaybeUninit<T>>,
        key1: F1,
        key2: F2,
        compare: F3,
    ) where
        K1: SortKey + Send + Sync,
        K2: SortKey + Send + Sync,
        F1: KeyFn<T, K1> + Send + Sync,
        F2: KeyFn<T, K2> + Send + Sync,
        F3: CmpFn<T> + Send + Sync;
}

#[cfg(not(feature = "allow_multithreading"))]
impl<T: Copy> TwoKeysAndCmpSort<T> for [T] {
    #[inline]
    fn sort_by_two_keys_then_by<K1, K2, F1, F2, F3>(
        &mut self,
        _: bool,
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
        if self.len() < BIN_SORT_MIN {
            sort_unstable_by_two_keys_then_by(self, key1, key2, compare);
            return;
        }
        self.ser_sort_by_two_keys_then_by_and_uninit_buffer(&mut Vec::new(), key1, key2, compare);
    }

    #[inline]
    fn sort_by_two_keys_then_by_and_buffer<K1, K2, F1, F2, F3>(
        &mut self,
        _: bool,
        reusable_buffer: &mut Vec<MaybeUninit<T>>,
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
        if self.len() < BIN_SORT_MIN {
            sort_unstable_by_two_keys_then_by(self, key1, key2, compare);
            return;
        }
        self.ser_sort_by_two_keys_then_by_and_uninit_buffer(reusable_buffer, key1, key2, compare);
    }
}

#[cfg(feature = "allow_multithreading")]
impl<T> TwoKeysAndCmpSort<T> for [T]
where
    T: Send + Sync + Copy,
{
    #[inline]
    fn sort_by_two_keys_then_by<K1, K2, F1, F2, F3>(
        &mut self,
        parallel: bool,
        key1: F1,
        key2: F2,
        compare: F3,
    ) where
        K1: SortKey + Send + Sync,
        K2: SortKey + Send + Sync,
        F1: KeyFn<T, K1> + Send + Sync,
        F2: KeyFn<T, K2> + Send + Sync,
        F3: CmpFn<T> + Send + Sync,
    {
        use crate::sort::parallel::slice_two_keys_cmp::TwoKeysBinSortCmpParallel;

        if self.len() < BIN_SORT_MIN {
            sort_unstable_by_two_keys_then_by(self, key1, key2, compare);
            return;
        }
        let mut reusable_buffer = Vec::new();
        if parallel {
            self.par_sort_by_two_keys_then_by(&mut reusable_buffer, key1, key2, compare);
        } else {
            self.ser_sort_by_two_keys_then_by_and_uninit_buffer(
                &mut reusable_buffer,
                key1,
                key2,
                compare,
            );
        }
    }

    #[inline]
    fn sort_by_two_keys_then_by_and_buffer<K1, K2, F1, F2, F3>(
        &mut self,
        parallel: bool,
        reusable_buffer: &mut Vec<MaybeUninit<T>>,
        key1: F1,
        key2: F2,
        compare: F3,
    ) where
        K1: SortKey + Send + Sync,
        K2: SortKey + Send + Sync,
        F1: KeyFn<T, K1> + Send + Sync,
        F2: KeyFn<T, K2> + Send + Sync,
        F3: CmpFn<T> + Send + Sync,
    {
        use crate::sort::parallel::slice_two_keys_cmp::TwoKeysBinSortCmpParallel;

        if self.len() < BIN_SORT_MIN {
            sort_unstable_by_two_keys_then_by(self, key1, key2, compare);
            return;
        }
        if parallel {
            self.par_sort_by_two_keys_then_by(reusable_buffer, key1, key2, compare);
        } else {
            self.ser_sort_by_two_keys_then_by_and_uninit_buffer(
                reusable_buffer,
                key1,
                key2,
                compare,
            );
        }
    }
}

#[inline]
pub(crate) fn sort_unstable_by_two_keys_then_by<T, K1, K2, F1, F2, F3>(
    slice: &mut [T],
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
    slice.sort_unstable_by(|a, b| {
        key1(a)
            .cmp(&key1(b))
            .then(key2(a).cmp(&key2(b)))
            .then(compare(a, b))
    });
}
