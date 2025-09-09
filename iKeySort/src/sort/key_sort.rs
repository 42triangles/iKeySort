use crate::sort::key::{KeyFn, SortKey};
use crate::sort::parallel::slice_one_key::OneKeyBinSortParallel;
use crate::sort::parallel::slice_two_keys::TwoKeysBinSortParallel;
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;

const BIN_SORT_MIN: usize = 64;

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
        if self.len() < BIN_SORT_MIN {
            self.sort_unstable_by_key(key);
            return;
        }
        if parallel {
            self.par_sort_by_one_key(key);
        } else {
            self.ser_sort_by_one_key(key);
        }
    }

    #[inline]
    fn sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        parallel: bool,
        key1: F1,
        key2: F2,
    ) {
        if self.len() < BIN_SORT_MIN {
            sort_unstable_by_two_keys(self, key1, key2);
            return;
        }
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
        if self.len() < BIN_SORT_MIN {
            self.sort_unstable_by_key(key);
            return;
        }
        self.ser_sort_by_one_key(key);
    }

    #[inline]
    fn sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        parallel: bool,
        key1: F1,
        key2: F2,
    ) {
        if self.len() < BIN_SORT_MIN {
            sort_unstable_by_two_keys(self, key1, key2);
            return;
        }
        self.ser_sort_by_two_keys(key1, key2);
    }
}

fn sort_unstable_by_two_keys<T, K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(slice: &mut [T], key1: F1, key2: F2) {
    slice.sort_unstable_by(|a, b| key1(a).cmp(&key1(b)).then(key2(a).cmp(&key2(b))));
}