use crate::sort::key::{KeyFn, SortKey};
use crate::sort::min_max::MinMax;

#[derive(Debug, Clone)]
pub struct BinLayout<K: Copy> {
    pub(crate) min_key: K,
    pub(crate) max_key: K,
    pub(crate) power: usize,
    bin_width_is_one: bool,
}

pub const MAX_BINS_POWER: u32 = 8;
pub const MAX_BINS_COUNT: usize = 1 << MAX_BINS_POWER;

impl<K: SortKey> BinLayout<K> {
    #[inline(always)]
    pub(super) fn bin_width_is_one(&self) -> bool {
        self.bin_width_is_one
    }

    #[inline(always)]
    pub fn index(&self, value: K) -> usize {
        let offset = value.difference(self.min_key);
        offset >> self.power
    }

    #[inline(always)]
    pub fn count(&self) -> usize {
        self.index(self.max_key) + 1
    }

    #[inline(always)]
    pub(crate) fn new(min_key: K, max_key: K, max_bins_count: usize) -> BinLayout<K> {
        let length = max_key.difference(min_key);
        if length < max_bins_count {
            return Self {
                min_key,
                max_key,
                power: 0,
                bin_width_is_one: true,
            };
        }

        let scale = (length + 1).ilog2_ceil();
        let power = scale.saturating_sub(max_bins_count.ilog2()) as usize;

        Self {
            min_key,
            max_key,
            power,
            bin_width_is_one: false,
        }
    }

    #[inline(always)]
    pub fn with_keys<T, F: KeyFn<T, K>>(array: &[T], key: F) -> Option<Self> {
        if array.is_empty() {
            return None;
        }

        let (min_key, max_key) = array.min_max(key);

        if min_key == max_key {
            return None;
        }

        Some(Self::new(min_key, max_key, MAX_BINS_COUNT))
    }
}

trait Log2 {
    fn ilog2_ceil(&self) -> u32;
}

impl Log2 for usize {
    #[inline(always)]
    fn ilog2_ceil(&self) -> u32 {
        let floor = self.ilog2();
        if self.is_power_of_two() {
            floor
        } else {
            floor + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sort::bin_layout::{BinLayout, Log2, MAX_BINS_COUNT};

    #[test]
    fn test_0() {
        let layout = BinLayout::<i32>::new(0i32, 3i32, MAX_BINS_COUNT);
        assert_eq!(layout.power, 0);
    }

    #[test]
    fn test_1() {
        let layout = BinLayout::<i32>::new(0, 255, MAX_BINS_COUNT);

        assert_eq!(layout.power, 0);
    }

    #[test]
    fn test_2() {
        let layout = BinLayout::<i32>::new(0, 256, MAX_BINS_COUNT);

        assert_eq!(layout.power, 1);
    }

    #[test]
    fn test_log2_0() {
        assert_eq!(1usize.ilog2_ceil(), 0);
        assert_eq!(2usize.ilog2_ceil(), 1);
        assert_eq!(3usize.ilog2_ceil(), 2);
        assert_eq!(4usize.ilog2_ceil(), 2);
        assert_eq!(5usize.ilog2_ceil(), 3);
    }
}
