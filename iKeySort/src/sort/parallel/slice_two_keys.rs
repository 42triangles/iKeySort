use crate::sort::key::{KeyFn, SortKey};
use crate::sort::parallel::cpu_count::CPUCount;
use crate::sort::parallel::fragment::Fragment;
use crate::sort::parallel::fragmentation::Fragmentation;
use crate::sort::parallel::presort::PreSort;
use crate::sort::serial::slice_two_keys::TwoKeysBinSortSerial;
use rayon::prelude::*;
use crate::sort::parallel::slice_one_key::OneKeyBinSortParallel;


pub(crate) trait TwoKeysBinSortParallel<T> {
    fn par_sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        key1: F1,
        key2: F2,
    );
}

impl<T: Copy + Send + Sync> TwoKeysBinSortParallel<T> for [T] {
    fn par_sort_by_two_keys<K: SortKey, F1: KeyFn<T, K>, F2: KeyFn<T, K>>(
        &mut self,
        key1: F1,
        key2: F2,
    ) {
        if self.is_empty() {
            return;
        }

        let cpu =if let Some(count) = CPUCount::should_parallel(self.len()) {
            count
        } else {
            self.ser_sort_by_two_keys(key1, key2);
            return;
        };

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

#[cfg(test)]
mod tests {
    use crate::sort::parallel::slice_two_keys::TwoKeysBinSortParallel;

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
        arr.par_sort_by_two_keys(|a| a.0, |a| a.1);
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
