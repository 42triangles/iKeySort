use crate::sort::parallel::fragment::Fragment;
use std::mem::MaybeUninit;

pub(super) trait Fragmentation<T> {
    fn fragment_by_count<'a>(
        &'a mut self,
        buffer: &'a mut [MaybeUninit<T>],
        count: usize,
    ) -> Vec<Fragment<'a, T>>;

    fn fragment_by_marks<'a>(
        &'a mut self,
        buffer: &'a mut [MaybeUninit<T>],
        marks: &[usize],
    ) -> Vec<Fragment<'a, T>>;
}

impl<T> Fragmentation<T> for [T] {
    #[inline]
    fn fragment_by_count<'a>(
        &'a mut self,
        buffer: &'a mut [MaybeUninit<T>],
        count: usize,
    ) -> Vec<Fragment<'a, T>> {
        debug_assert_eq!(self.len(), buffer.len());

        let (capacity, step_len) = if self.len() < count {
            (self.len(), 1)
        } else {
            let step_len = self.len().div_ceil(count);
            let count = self.len().div_ceil(step_len);
            (count, step_len)
        };

        let mut frags = Vec::with_capacity(capacity);
        let mut base = 0;

        let mut src = self;
        let mut buf = buffer;

        for _ in 0..capacity.saturating_sub(1) {
            let (left_src, right_src) = src.split_at_mut(step_len);
            let (left_buf, right_buf) = buf.split_at_mut(step_len);

            frags.push(Fragment {
                base,
                src: left_src,
                buf: left_buf,
            });

            src = right_src;
            buf = right_buf;

            base += step_len;
        }

        frags.push(Fragment { base, src, buf });

        frags
    }

    #[inline]
    fn fragment_by_marks<'a>(
        &'a mut self,
        buffer: &'a mut [MaybeUninit<T>],
        marks: &[usize],
    ) -> Vec<Fragment<'a, T>> {
        debug_assert_eq!(self.len(), buffer.len());

        let mut frags = Vec::with_capacity(marks.len() + 1);

        let mut src = self;
        let mut buf = buffer;

        let mut base = 0;
        for &m in marks.iter() {
            debug_assert!(m >= base, "marks must be non-decreasing");
            debug_assert!(m <= base + src.len(), "mark {m} out of bounds");

            let md = m - base;
            let (left_src, right_src) = src.split_at_mut(md);
            let (left_buf, right_buf) = buf.split_at_mut(md);

            frags.push(Fragment {
                base,
                src: left_src,
                buf: left_buf,
            });

            src = right_src;
            buf = right_buf;

            base = m;
        }

        if !src.is_empty() {
            frags.push(Fragment { base, src, buf });
        }

        frags
    }
}
