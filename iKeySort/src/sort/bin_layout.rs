use crate::sort::key::{KeyFn, SortKey};
use crate::sort::log2::Log2;
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

#[cfg(test)]
mod tests {
    use crate::sort::bin_layout::{BinLayout, MAX_BINS_COUNT};

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
}
