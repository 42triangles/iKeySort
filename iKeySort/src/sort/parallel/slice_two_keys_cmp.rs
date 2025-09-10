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
    fn par_sort_by_two_keys_then_by<K1, K2, F1, F2, F3>(&mut self, key1: F1, key2: F2, compare: F3)
    where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
        F3: CmpFn<T>;
}

impl<T: Copy + Send + Sync> TwoKeysBinSortCmpParallel<T> for [T] {
    fn par_sort_by_two_keys_then_by<K1, K2, F1, F2, F3>(&mut self, key1: F1, key2: F2, compare: F3)
    where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
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
    fn sort_by_two_keys_then_by<K1, K2, F1, F2, F3>(&mut self, key1: F1, key2: F2, compare: F3)
    where
        K1: SortKey,
        K2: SortKey,
        F1: KeyFn<T, K1>,
        F2: KeyFn<T, K2>,
        F3: CmpFn<T>,
    {
        self.src
            .sort_by_two_keys_and_uninit_buffer_then_by(self.buf, key1, key2, compare);
    }
}
#[cfg(test)]
mod tests {
    use crate::sort::parallel::slice_two_keys_cmp::TwoKeysBinSortCmpParallel;

    #[test]
    fn test_0() {
        test(2);
    }

    #[test]
    fn test_1() {
        test(5);
    }

    #[test]
    fn test_2() {
        test(10);
    }

    #[test]
    fn test_3() {
        test(20);
    }

    #[test]
    fn test_4() {
        test(40);
    }

    #[test]
    fn test_5() {
        test(100);
    }

    fn test(count: usize) {
        let mut org: Vec<_> = reversed_2d_array(count);
        let mut arr = org.clone();
        arr.par_sort_by_two_keys_then_by(|a| a.0, |a| a.1, |a, b| a.2.cmp(&b.2));
        org.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
        assert_eq!(arr, org);
    }

    fn reversed_2d_array(count: usize) -> Vec<(u32, i32, i32)> {
        let mut arr = Vec::with_capacity(count * count * count);
        for i in (0..count as u32).rev() {
            for x in (0..count as i32).rev() {
                for y in (0..count as i32).rev() {
                    arr.push((i, x, y))
                }
            }
        }

        arr
    }
}