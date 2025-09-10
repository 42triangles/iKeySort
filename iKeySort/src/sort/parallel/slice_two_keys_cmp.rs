use crate::sort::key::{CmpFn, KeyFn, SortKey};
use crate::sort::parallel::cpu_count::CPUCount;
use crate::sort::parallel::fragment::Fragment;
use crate::sort::parallel::fragmentation::Fragmentation;
use crate::sort::parallel::presort::PreSort;
use crate::sort::parallel::slice_one_key::OneKeyBinSortParallel;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;
use rayon::prelude::*;
use crate::sort::serial::slice_two_keys_cmp::TwoKeysBinSortCmpSerial;

pub(crate) trait TwoKeysBinSortCmpParallel<T> {
    fn par_sort_by_two_keys_then_by<K, F1, F2, F3>(&mut self, key1: F1, key2: F2, compare: F3)
    where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: KeyFn<T, K>,
        F3: CmpFn<T>;
}

impl<T: Copy + Send + Sync> TwoKeysBinSortCmpParallel<T> for [T] {
    fn par_sort_by_two_keys_then_by<K, F1, F2, F3>(&mut self, key1: F1, key2: F2, compare: F3)
    where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: KeyFn<T, K>,
        F3: CmpFn<T>,
    {
        if self.is_empty() {
            return;
        }

        let cpu = if let Some(count) = CPUCount::should_parallel(self.len()) {
            count
        } else {
            self.ser_sort_by_two_keys(key1, key2);
            return;
        };

        if let Some((marks, mut buffer)) = self.par_pre_sort(cpu, key1) {
            self.fragment_by_marks(&mut buffer, &marks)
                .par_iter_mut()
                .for_each(|f| f.sort_by_two_keys_then_by(key1, key2, compare));
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
    fn sort_by_two_keys_then_by<K, F1, F2, F3>(&mut self, key1: F1, key2: F2, compare: F3)
    where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: KeyFn<T, K>,
        F3: CmpFn<T>,
    {
        self.src
            .sort_by_two_keys_and_uninit_buffer_then_by(self.buf, key1, key2, compare);
    }
}
