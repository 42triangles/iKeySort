use rayon::prelude::*;
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::parallel::cpu_count::CPUCount;
use crate::sort::parallel::fragment::Fragment;
use crate::sort::parallel::fragmentation::Fragmentation;
use crate::sort::parallel::presort::PreSort;
use crate::sort::parallel::slice_one_key::OneKeyBinSortParallel;
use crate::sort::serial::slice_two_keys::{TwoKeysBinSortSerial, TwoKeysBufferBinSortSerial};

pub trait TwoKeysBinSortParallel<T> {
    fn par_sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        key1: F1,
        key2: F2,
    );
}

impl<T: Copy + Send + Sync> TwoKeysBinSortParallel<T> for [T] {
    #[inline]
    fn par_sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        key1: F1,
        key2: F2,
    ) {
        if self.is_empty() {
            return;
        }

        let cpu = CPUCount::count();
        if cpu == 1 {
            self.sort_by_two_keys(key1, key2);
            return;
        }
        if let Some((marks, mut buffer)) = self.par_pre_sort(cpu, key1) {
            self.fragment_by_marks(&mut buffer, &marks)
                .par_iter_mut()
                .for_each(|f| f.sort_by_two_keys(key1, key2));
        } else {
            // array is flat by key1
            self.par_sort_by_one_key(key2)
        }
    }
}

impl<T> Fragment<'_, T>
where
    T: Send + Copy,
{
    #[inline]
    fn sort_by_two_keys<K, F1, F2>(&mut self, key1: F1, key2: F2)
    where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: KeyFn<T, K>,
    {
        self.slice
            .sort_by_two_keys_and_buffer(self.buffer, key1, key2);
    }
}
