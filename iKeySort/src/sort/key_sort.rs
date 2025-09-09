use crate::sort::key::{KeyFn, SortKey};
use crate::sort::parallel::slice_one_key::OneKeyBinSortParallel;
use crate::sort::parallel::slice_two_keys::TwoKeysBinSortParallel;
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;

pub trait KeySort<T> {
    fn sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, parallel: bool, key: F);
    fn sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        parallel: bool,
        key1: F1,
        key2: F2,
    );
}

#[cfg(feature = "allow_multithreading")]
impl<T: Copy + Send + Sync> KeySort<T> for [T] {
    #[inline]
    fn sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, parallel: bool, key: F) {
        if parallel {
            self.par_sort_by_one_key(key);
        } else {
            self.ser_sort_by_one_key(key);
        }
    }

    #[inline]
    fn sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(&mut self, parallel: bool, key1: F1, key2: F2) {
        if parallel {
            self.par_sort_by_two_keys(key1, key2);
        } else {
            self.ser_sort_by_two_keys(key1, key2);
        }
    }
}

#[cfg(not(feature = "allow_multithreading"))]
impl<T: Copy> KeySort<T> for [T] {
    #[inline]
    fn sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, parallel: bool, key: F) {
        self.ser_sort_by_one_key(key);
    }

    #[inline]
    fn sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(&mut self, parallel: bool, key1: F1, key2: F2) {
        self.ser_sort_by_two_keys(key1, key2);
    }
}