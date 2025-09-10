pub(crate) trait Log2 {
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
    use crate::sort::log2::Log2;

    #[test]
    fn test_0() {
        assert_eq!(1usize.ilog2_ceil(), 0);
        assert_eq!(2usize.ilog2_ceil(), 1);
        assert_eq!(3usize.ilog2_ceil(), 2);
        assert_eq!(4usize.ilog2_ceil(), 2);
        assert_eq!(5usize.ilog2_ceil(), 3);
    }
}
