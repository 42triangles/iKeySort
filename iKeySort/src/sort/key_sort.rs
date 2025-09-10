use crate::sort::key::{CmpFn, KeyFn, SortKey};
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;
use crate::sort::serial::slice_one_key_cmp::OneKeyBinSortCmpSerial;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;
use crate::sort::serial::slice_two_keys_cmp::TwoKeysBinSortCmpSerial;

pub(crate) const BIN_SORT_MIN: usize = 64;

#[cfg(not(feature = "allow_multithreading"))]
pub trait KeySort<T> {
    fn sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, parallel: bool, key: F);
    fn sort_by_two_keys<K1, K2, F1, F2>(&mut self, parallel: bool, key1: F1, key2: F2)
    where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>;

    fn sort_by_one_key_then_by<K, F1, F2>(&mut self, parallel: bool, key: F1, compare: F2)
    where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>;

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
}

#[cfg(feature = "allow_multithreading")]
pub trait KeySort<T> {
    fn sort_by_one_key<K, F>(&mut self, parallel: bool, key: F)
    where
        K: SortKey + Send + Sync,
        F: KeyFn<T, K> + Send + Sync;
    fn sort_by_two_keys<K1, K2, F1, F2>(&mut self, parallel: bool, key1: F1, key2: F2)
    where
        K1: SortKey + Send + Sync,
        K2: SortKey + Send + Sync,
        F1: KeyFn<T, K1> + Send + Sync,
        F2: KeyFn<T, K2> + Send + Sync;

    fn sort_by_one_key_then_by<K, F1, F2>(&mut self, parallel: bool, key: F1, compare: F2)
    where
        K: SortKey + Send + Sync,
        F1: KeyFn<T, K> + Send + Sync,
        F2: CmpFn<T> + Send + Sync;

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
}

#[cfg(feature = "allow_multithreading")]
impl<T: Send + Sync + Copy> KeySort<T> for [T] {
    #[inline]
    fn sort_by_one_key<K, F>(&mut self, parallel: bool, key: F)
    where
        K: SortKey + Send + Sync,
        F: KeyFn<T, K> + Send + Sync,
    {
        use crate::sort::parallel::slice_one_key::OneKeyBinSortParallel;

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
        if parallel {
            self.par_sort_by_two_keys(key1, key2);
        } else {
            self.ser_sort_by_two_keys(key1, key2);
        }
    }

    #[inline]
    fn sort_by_one_key_then_by<K, F1, F2>(&mut self, parallel: bool, key: F1, compare: F2)
    where
        K: SortKey + Send + Sync,
        F1: KeyFn<T, K> + Send + Sync,
        F2: CmpFn<T> + Send + Sync,
    {
        use crate::sort::parallel::slice_one_key_cmp::OneKeyBinSortCmpParallel;

        if self.len() < BIN_SORT_MIN {
            sort_unstable_by_one_key_then_by(self, key, compare);
            return;
        }
        if parallel {
            self.par_sort_by_one_key_then_by(key, compare);
        } else {
            self.ser_sort_by_one_key_then_by(key, compare);
        }
    }

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
        if parallel {
            self.par_sort_by_two_keys_then_by(key1, key2, compare);
        } else {
            self.ser_sort_by_two_keys_then_by(key1, key2, compare);
        }
    }
}

#[cfg(not(feature = "allow_multithreading"))]
impl<T: Copy> KeySort<T> for [T] {
    #[inline]
    fn sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, _: bool, key: F) {
        if self.len() < BIN_SORT_MIN {
            self.sort_unstable_by_key(key);
            return;
        }
        self.ser_sort_by_one_key(key);
    }

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
        self.ser_sort_by_two_keys(key1, key2);
    }

    fn sort_by_one_key_then_by<K, F1, F2>(&mut self, _: bool, key: F1, compare: F2)
    where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>,
    {
        if self.len() < BIN_SORT_MIN {
            sort_unstable_by_one_key_then_by(self, key, compare);
            return;
        }
        self.ser_sort_by_one_key_then_by(key, compare);
    }

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
        self.ser_sort_by_two_keys_then_by(key1, key2, compare);
    }
}

#[inline]
fn sort_unstable_by_two_keys<T, K1: SortKey, K2: SortKey, F1: KeyFn<T, K1>, F2: KeyFn<T, K2>>(
    slice: &mut [T],
    key1: F1,
    key2: F2,
) {
    slice.sort_unstable_by(|a, b| key1(a).cmp(&key1(b)).then(key2(a).cmp(&key2(b))));
}

#[inline]
fn sort_unstable_by_one_key_then_by<T, K, F1, F2>(slice: &mut [T], key: F1, compare: F2)
where
    K: SortKey,
    F1: KeyFn<T, K>,
    F2: CmpFn<T>,
{
    slice.sort_unstable_by(|a, b| key(a).cmp(&key(b)).then(compare(a, b)));
}

#[inline]
fn sort_unstable_by_two_keys_then_by<T, K1, K2, F1, F2, F3>(
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
