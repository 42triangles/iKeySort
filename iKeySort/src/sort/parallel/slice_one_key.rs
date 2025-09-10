use crate::sort::key::{KeyFn, SortKey};
use crate::sort::parallel::cpu_count::CPUCount;
use crate::sort::parallel::fragment::Fragment;
use crate::sort::parallel::fragmentation::Fragmentation;
use crate::sort::parallel::presort::PreSort;
use crate::sort::serial::slice_one_key::OneKeyBinSortSerial;
use rayon::prelude::*;

pub(crate) trait OneKeyBinSortParallel<T> {
    fn par_sort_by_one_key<K, F>(&mut self, key: F)
    where
        K: SortKey + Send + Sync,
        F: KeyFn<T, K> + Send + Sync;
}

impl<T: Copy + Send + Sync> OneKeyBinSortParallel<T> for [T] {
    #[inline]
    fn par_sort_by_one_key<K, F>(&mut self, key: F)
    where
        K: SortKey + Send + Sync,
        F: KeyFn<T, K> + Send + Sync,
    {
        if self.is_empty() {
            return;
        }

        let cpu = if let Some(count) = CPUCount::should_parallel(self.len()) {
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

#[cfg(test)]
mod tests {
    use crate::sort::parallel::slice_one_key::OneKeyBinSortParallel;

    #[test]
    fn test_0() {
        test(10);
    }

    #[test]
    fn test_1() {
        test(100);
    }

    #[test]
    fn test_2() {
        test(1_000);
    }

    #[test]
    fn test_3() {
        test(10_000);
    }

    #[test]
    fn test_4() {
        test(100_000);
    }

    #[test]
    fn test_5() {
        test(1000_000);
    }

    fn test(count: usize) {
        let mut org: Vec<_> = (0..count).rev().collect();
        let mut arr = org.clone();
        arr.par_sort_by_one_key(|&a| a);
        org.sort_unstable();
        assert_eq!(arr, org);
    }
}
