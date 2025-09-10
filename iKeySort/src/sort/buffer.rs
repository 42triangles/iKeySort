use std::mem::MaybeUninit;
use std::ops::Range;
use std::{ptr, slice};

pub(crate) trait MaybeUninitInit<T> {
    fn assume_init_slice_mut(&mut self) -> &mut [T];
}

pub(crate) trait CopyFromNotOverlap<T> {
    fn copy_from_not_overlap(&mut self, buffer: &mut [T]);
    fn copy_to_range_from_not_overlap(&mut self, buffer: &mut [T], range: Range<usize>);
}

pub(crate) trait CopyNotOverlapValue<T> {
    fn copy_value_from(&mut self, src: &[T], index: usize);
}

impl<T> MaybeUninitInit<T> for [MaybeUninit<T>] {
    #[inline(always)]
    fn assume_init_slice_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr() as *mut T, self.len()) }
    }
}

impl<T: Copy> CopyNotOverlapValue<T> for [T] {
    #[inline(always)]
    fn copy_value_from(&mut self, src: &[T], index: usize) {
        unsafe {
            let dst = self.get_unchecked_mut(index);
            let val = src.get_unchecked(index);
            *dst = *val;
        }
    }
}

impl<T> CopyFromNotOverlap<T> for [T] {
    #[inline(always)]
    fn copy_from_not_overlap(&mut self, buffer: &mut [T]) {
        unsafe {
            let dst_ptr = self.as_mut_ptr();
            let src_ptr = buffer.as_ptr();

            ptr::copy_nonoverlapping(src_ptr, dst_ptr, self.len());
        }
    }

    #[inline(always)]
    fn copy_to_range_from_not_overlap(&mut self, buffer: &mut [T], range: Range<usize>) {
        debug_assert_eq!(range.len(), buffer.len());
        let dst = unsafe { self.get_unchecked_mut(range) };
        dst.copy_from_not_overlap(buffer);
    }
}

pub(crate) trait DoubleRangeSlices<T> {
    fn mut_slices<'a>(
        &self,
        slice1: &'a mut [T],
        slice2: &'a mut [T],
    ) -> (&'a mut [T], &'a mut [T]);
}

impl<T> DoubleRangeSlices<T> for Range<usize> {
    #[inline(always)]
    fn mut_slices<'a>(
        &self,
        slice1: &'a mut [T],
        slice2: &'a mut [T],
    ) -> (&'a mut [T], &'a mut [T]) {
        unsafe {
            let sub1 = slice1.get_unchecked_mut(self.clone());
            let sub2 = slice2.get_unchecked_mut(self.clone());
            (sub1, sub2)
        }
    }
}
