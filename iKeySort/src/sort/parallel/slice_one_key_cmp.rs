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

#[cfg(test)]
mod tests {
    use crate::sort::parallel::slice_one_key_cmp::OneKeyBinSortCmpParallel;

    #[test]
    fn test_0() {
        test(2);
    }

    #[test]
    fn test_1() {
        test(10);
    }

    #[test]
    fn test_2() {
        test(30);
    }

    #[test]
    fn test_3() {
        test(100);
    }

    #[test]
    fn test_4() {
        test(300);
    }

    #[test]
    fn test_5() {
        test(1000);
    }

    fn test(count: usize) {
        let mut org: Vec<_> = reversed_2d_array(count);
        let mut arr = org.clone();
        arr.par_sort_by_one_key_then_by(|a| a.0, |a, b| a.1.cmp(&b.1));
        org.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        assert_eq!(arr, org);
    }

    fn reversed_2d_array(count: usize) -> Vec<(i32, i32)> {
        let mut arr = Vec::with_capacity(count * count);
        for x in (0..count as i32).rev() {
            for y in (0..count as i32).rev() {
                arr.push((x, y))
            }
        }

        arr
    }
}