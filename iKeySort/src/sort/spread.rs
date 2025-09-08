use alloc::slice;
use core::mem::MaybeUninit;
use core::ptr;
use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mapper::Mapper;

impl<K: SortKey> BinLayout<K> {
    #[inline(always)]
    pub(crate) fn spread_with_buffer<T: Copy, F: KeyFn<T, K>>(
        &self,
        slice: &mut [T],
        buffer: &mut [MaybeUninit<T>],
        key: F,
    ) -> Mapper {
        let mut mapper = Mapper::new(self.count());
        for a in slice.iter() {
            mapper.inc_bin_count(self.index(key(a)));
        }

        mapper.init_indices();

        for val in slice.iter() {
            let index = mapper.next_index(self.index(key(val)));
            unsafe {
                buffer.get_unchecked_mut(index).write(*val);
            }
        }
        unsafe {
            let buffer_init = slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut T, buffer.len());

            let dst_ptr = slice.as_mut_ptr();
            let src_ptr = buffer_init.as_ptr();

            ptr::copy_nonoverlapping(src_ptr, dst_ptr, slice.len());
        }

        mapper
    }
}
