use crate::sort::key::{KeyFn, SortKey};
use crate::sort::parallel::cpu_count::CPUCount;
use crate::sort::parallel::fragment::Fragment;
use crate::sort::parallel::fragmentation::Fragmentation;
use crate::sort::parallel::presort::PreSort;
use crate::sort::parallel::slice_one_key::OneKeyBinSortParallel;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;
use rayon::prelude::*;

pub(crate) trait TwoKeysBinSortParallel<T> {
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

        #[cfg(debug_assertions)]
        const MIN_PAR_LEN: usize = 64_000;

        #[cfg(not(debug_assertions))]
        const MIN_PAR_LEN: usize = 0;

        let cpu = CPUCount::count();
        if cpu == 1 || self.len() < MIN_PAR_LEN {
            self.ser_sort_by_two_keys(key1, key2);
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
        self.src
            .sort_by_two_keys_and_uninit_buffer(self.buf, key1, key2);
    }
}
