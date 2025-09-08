use crate::sort::bin_layout::{BinLayout, MAX_BINS_COUNT};
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::min_max::MinMax;
use crate::sort::parallel::fragment::{Fragment, IdRange};
use crate::sort::parallel::fragmentation::Fragmentation;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use std::mem::MaybeUninit;
use std::ops::Range;
use std::ptr;

pub(super) trait PreSort<T> {
    fn par_pre_sort<K: SortKey, F: KeyFn<T, K>>(
        &mut self,
        cpu: usize,
        key: F,
    ) -> Option<(Vec<usize>, Vec<MaybeUninit<T>>)>;
}

impl<T: Copy + Send + Sync> PreSort<T> for [T] {
    fn par_pre_sort<K: SortKey, F: KeyFn<T, K>>(
        &mut self,
        cpu: usize,
        key: F,
    ) -> Option<(Vec<usize>, Vec<MaybeUninit<T>>)> {
        let (min_key, max_key) = self.par_min_max(key);

        if min_key == max_key {
            // array is flat
            return None;
        }
        debug_assert!(cpu > 1);
        let max_bins_count = cpu.saturating_mul(4).min(MAX_BINS_COUNT);

        let layout = BinLayout::new(min_key, max_key, max_bins_count);

        let mut buffer: Vec<MaybeUninit<T>> = Vec::with_capacity(self.len());
        unsafe {
            buffer.set_len(self.len());
        }

        let mut fragments = self.fragment_by_count(&mut buffer, cpu);

        let groups = layout.par_spread(&mut fragments, key);

        // at this time buffer contains semi sorted segments

        let src =
            unsafe { std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut T, buffer.len()) };

        let marks = copy_groups(self, src, groups);

        Some((marks, buffer))
    }
}

impl<K: SortKey> BinLayout<K> {
    fn par_spread<T, F>(&self, fragments: &mut [Fragment<T>], key: F) -> Vec<Vec<Range<usize>>>
    where
        T: Copy + Send + Sync,
        F: KeyFn<T, K>,
    {
        let bins_count = self.count();
        let frags_count = fragments.len();

        let id_ranges = fragments
            .par_iter_mut()
            .map(|f| f.spread(self.clone(), key))
            .reduce(
                || Vec::<IdRange>::with_capacity(frags_count * bins_count),
                |mut a, mut b| {
                    a.append(&mut b);
                    a
                },
            );

        let mut counter = vec![0usize; bins_count];
        for range in id_ranges.iter() {
            counter[range.index] += 1;
        }

        let mut groups: Vec<Vec<Range<usize>>> = (0..bins_count).map(|_| Vec::new()).collect();
        for (group, &count) in groups.iter_mut().zip(counter.iter()) {
            if count > 0 {
                *group = Vec::with_capacity(count);
            }
        }

        for id_range in id_ranges {
            unsafe {
                groups
                    .get_unchecked_mut(id_range.index)
                    .push(id_range.range);
            }
        }

        groups.retain(|g| !g.is_empty());
        groups
    }
}

#[inline]
fn copy_groups<T>(mut dst: &mut [T], src: &[T], mut groups: Vec<Vec<Range<usize>>>) -> Vec<usize> {
    let mut marks = Vec::with_capacity(groups.len());
    let last_group = if let Some(last) = groups.pop() {
        last
    } else {
        return marks;
    };

    let mut base = 0;
    for ranges in groups.iter() {
        let length = copy_ranges(dst, src, ranges);
        (_, dst) = dst.split_at_mut(length);

        base += length;
        marks.push(base);
    }

    let _ = copy_ranges(dst, src, &last_group);

    marks
}

#[inline]
fn copy_ranges<T>(dst: &mut [T], src: &[T], ranges: &[Range<usize>]) -> usize {
    let mut offset = 0;
    for range in ranges.iter() {
        unsafe {
            let dst_ptr = dst.as_mut_ptr().add(offset);
            let src_ptr = src.as_ptr().add(range.start);
            ptr::copy_nonoverlapping(src_ptr, dst_ptr, range.len());
        }

        offset += range.len();
    }

    offset
}
