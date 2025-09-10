use crate::sort::key::{CmpFn, KeyFn, SortKey};
use crate::sort::parallel::cpu_count::CPUCount;
use crate::sort::parallel::fragment::Fragment;
use crate::sort::parallel::fragmentation::Fragmentation;
use crate::sort::parallel::presort::PreSort;
use crate::sort::serial::slice_one_key_cmp::OneKeyBinSortCmpSerial;
use rayon::prelude::*;

pub(crate) trait OneKeyBinSortCmpParallel<T> {
    fn par_sort_by_one_key_then_by<K, F1, F2>(&mut self, key: F1, compare: F2)
    where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>;
}

impl<T: Copy + Send + Sync> OneKeyBinSortCmpParallel<T> for [T] {
    #[inline]
    fn par_sort_by_one_key_then_by<K, F1, F2>(&mut self, key: F1, compare: F2)
    where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>,
    {
        if self.is_empty() {
            return;
        }

        let cpu = if let Some(count) = CPUCount::should_parallel(self.len()) {
            count
        } else {
            self.ser_sort_by_one_key_then_by(key, compare);
            return;
        };

        if let Some((marks, mut buf)) = self.par_pre_sort(cpu, key) {
            self.fragment_by_marks(&mut buf, &marks)
                .par_iter_mut()
                .for_each(|f| f.sort_by_one_key_then_by(key, compare));
        }
    }
}

impl<T> Fragment<'_, T>
where
    T: Send + Copy,
{
    #[inline]
    fn sort_by_one_key_then_by<K, F1, F2>(&mut self, key: F1, compare: F2)
    where
        K: SortKey,
        F1: KeyFn<T, K>,
        F2: CmpFn<T>,
    {
        self.src.sort_by_one_key_and_uninit_buffer_then_by(self.buf, key, compare);
    }
}
