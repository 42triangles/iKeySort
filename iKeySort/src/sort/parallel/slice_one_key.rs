use crate::sort::key::{KeyFn, SortKey};
use crate::sort::parallel::cpu_count::CPUCount;
use crate::sort::parallel::fragment::Fragment;
use crate::sort::parallel::fragmentation::Fragmentation;
use crate::sort::parallel::presort::PreSort;
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;
use rayon::prelude::*;

pub(crate) trait OneKeyBinSortParallel<T> {
    fn par_sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, key: F);
}

impl<T: Copy + Send + Sync> OneKeyBinSortParallel<T> for [T] {
    #[inline]
    fn par_sort_by_one_key<K: SortKey, F: KeyFn<T, K>>(&mut self, key: F) {
        if self.is_empty() {
            return;
        }

        let cpu =if let Some(count) = CPUCount::should_parallel(self.len()) {
            count
        } else {
            self.ser_sort_by_one_key(key);
            return;
        };

        if let Some((marks, mut buf)) = self.par_pre_sort(cpu, key) {
            self.fragment_by_marks(&mut buf, &marks)
                .par_iter_mut()
                .for_each(|f| f.sort_by_one_key(key));
        }
    }
}

impl<T> Fragment<'_, T>
where
    T: Send + Copy,
{
    #[inline]
    fn sort_by_one_key<K, F>(&mut self, key: F)
    where
        K: SortKey,
        F: KeyFn<T, K>,
    {
        self.src.sort_by_one_key_and_uninit_buffer(self.buf, key);
    }
}