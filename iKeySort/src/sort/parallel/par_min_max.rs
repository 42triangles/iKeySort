use crate::sort::key::KeyFn;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

pub(super) trait ParMinMax<T> {
    fn par_min_max<K, F>(&self, key: F) -> (K, K)
    where
        K: Copy + Ord + Send + Sync,
        T: Copy + Send + Sync,
        F: KeyFn<T, K>;
}

impl<T> ParMinMax<T> for [T] {
    #[inline(always)]
    fn par_min_max<K, F>(&self, key: F) -> (K, K)
    where
        K: Copy + Ord + Send + Sync,
        T: Copy + Send + Sync,
        F: KeyFn<T, K>,
    {
        debug_assert!(!self.is_empty());
        let first_val = self.first().unwrap();
        let k0 = key(first_val);

        let (min_key, max_key) = self
            .par_iter()
            .map(|v| {
                let k = key(v);
                (k, k)
            })
            .reduce(|| (k0, k0), |a, b| (a.0.min(b.0), a.1.max(b.1)));

        (min_key, max_key)
    }
}
