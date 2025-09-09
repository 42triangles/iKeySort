use crate::sort::bin_layout::BinLayout;
use crate::sort::key::{KeyFn, SortKey};
use crate::sort::mapper::Mapper;
use core::mem::MaybeUninit;

impl<K: SortKey> BinLayout<K> {
    #[inline(always)]
    pub(crate) fn spread_with_uninit_buffer<T: Copy, F: KeyFn<T, K>>(
        &self,
        src: &mut [T],
        buf: &mut [MaybeUninit<T>],
        key: F,
    ) -> Mapper {
        let mut mapper = Mapper::new(self.count());
        for a in src.iter() {
            mapper.inc_bin_count(self.index(key(a)));
        }

        mapper.init_indices();

        for val in src.iter() {
            let index = mapper.next_index(self.index(key(val)));
            unsafe {
                buf.get_unchecked_mut(index).write(*val);
            }
        }

        mapper
    }

    #[inline(always)]
    pub(crate) fn spread_with_buffer<T: Copy, F: KeyFn<T, K>>(
        &self,
        src: &mut [T],
        buf: &mut [T],
        key: F,
    ) -> Mapper {
        let mut mapper = Mapper::new(self.count());
        for a in src.iter() {
            mapper.inc_bin_count(self.index(key(a)));
        }

        mapper.init_indices();

        for val in src.iter() {
            let index = mapper.next_index(self.index(key(val)));
            unsafe {
                *buf.get_unchecked_mut(index) = *val;
            }
        }

        mapper
    }
}
