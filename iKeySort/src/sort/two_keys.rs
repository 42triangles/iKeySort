use crate::sort::bin_layout::BIN_SORT_MIN;
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;
use alloc::vec::Vec;
use core::mem::MaybeUninit;

#[cfg(not(feature = "allow_multithreading"))]
pub trait TwoKeysSort<T> {
    fn sort_by_two_keys<K1, K2, F1, F2>(&mut self, parallel: bool, key1: F1, key2: F2)
    where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>;

    fn sort_by_two_keys_and_buffer<K1, K2, F1, F2>(
        &mut self,
        parallel: bool,
        reusable_buffer: &mut Vec<MaybeUninit<T>>,
        key1: F1,
        key2: F2,
    ) where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>;
}

#[cfg(feature = "allow_multithreading")]
pub trait TwoKeysSort<T> {
    fn sort_by_two_keys<K1, K2, F1, F2>(&mut self, parallel: bool, key1: F1, key2: F2)
    where
        K1: SortKey + Send + Sync,
        K2: SortKey + Send + Sync,
        F1: KeyFn<T, K1> + Send + Sync,
        F2: KeyFn<T, K2> + Send + Sync;

    fn sort_by_two_keys_and_buffer<K1, K2, F1, F2>(
        &mut self,
        parallel: bool,
        reusable_buffer: &mut Vec<MaybeUninit<T>>,
        key1: F1,
        key2: F2,
    ) where
        K1: SortKey + Send + Sync,
        K2: SortKey + Send + Sync,
        F1: KeyFn<T, K1> + Send + Sync,
        F2: KeyFn<T, K2> + Send + Sync;
}

#[cfg(not(feature = "allow_multithreading"))]
impl<T: Copy> TwoKeysSort<T> for [T] {
    #[inline]
    fn sort_by_two_keys<K1, K2, F1, F2>(&mut self, _: bool, key1: F1, key2: F2)
    where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
    {
        if self.len() < BIN_SORT_MIN {
            sort_unstable_by_two_keys(self, key1, key2);
            return;
        }
        self.ser_sort_by_two_keys_and_uninit_buffer(&mut Vec::new(), key1, key2);
    }

    #[inline]
    fn sort_by_two_keys_and_buffer<K1, K2, F1, F2>(
        &mut self,
        _: bool,
        reusable_buffer: &mut Vec<MaybeUninit<T>>,
        key1: F1,
        key2: F2,
    ) where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
    {
        if self.len() < BIN_SORT_MIN {
            sort_unstable_by_two_keys(self, key1, key2);
            return;
        }
        self.ser_sort_by_two_keys_and_uninit_buffer(reusable_buffer, key1, key2);
    }
}

#[cfg(feature = "allow_multithreading")]
impl<T> TwoKeysSort<T> for [T]
where
    T: Send + Sync + Copy,
{
    #[inline]
    fn sort_by_two_keys<K1, K2, F1, F2>(&mut self, parallel: bool, key1: F1, key2: F2)
    where
        K1: SortKey + Send + Sync,
        K2: SortKey + Send + Sync,
        F1: KeyFn<T, K1> + Send + Sync,
        F2: KeyFn<T, K2> + Send + Sync,
    {
        use crate::sort::parallel::slice_two_keys::TwoKeysBinSortParallel;

        if self.len() < BIN_SORT_MIN {
            sort_unstable_by_two_keys(self, key1, key2);
            return;
        }
        let mut reusable_buffer = Vec::new();
        if parallel {
            self.par_sort_by_two_keys(&mut reusable_buffer, key1, key2);
        } else {
            self.ser_sort_by_two_keys_and_uninit_buffer(&mut reusable_buffer, key1, key2);
        }
    }

    #[inline]
    fn sort_by_two_keys_and_buffer<K1, K2, F1, F2>(
        &mut self,
        parallel: bool,
        reusable_buffer: &mut Vec<MaybeUninit<T>>,
        key1: F1,
        key2: F2,
    ) where
        K1: SortKey + Send + Sync,
        K2: SortKey + Send + Sync,
        F1: KeyFn<T, K1> + Send + Sync,
        F2: KeyFn<T, K2> + Send + Sync,
    {
        use crate::sort::parallel::slice_two_keys::TwoKeysBinSortParallel;

        if self.len() < BIN_SORT_MIN {
            sort_unstable_by_two_keys(self, key1, key2);
            return;
        }
        if parallel {
            self.par_sort_by_two_keys(reusable_buffer, key1, key2);
        } else {
            self.ser_sort_by_two_keys_and_uninit_buffer(reusable_buffer, key1, key2);
        }
    }
}

#[inline]
pub(crate) fn sort_unstable_by_two_keys<T, K1, K2, F1, F2>(slice: &mut [T], key1: F1, key2: F2)
where
    K1: SortKey,
    K2: SortKey,
    F1: KeyFn<T, K1>,
    F2: KeyFn<T, K2>,
{
    slice.sort_unstable_by(|a, b| key1(a).cmp(&key1(b)).then(key2(a).cmp(&key2(b))))
}
