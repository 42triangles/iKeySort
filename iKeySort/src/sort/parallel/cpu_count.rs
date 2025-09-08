pub(super) struct CPUCount;

impl CPUCount {
    #[inline]
    pub(super) fn count() -> usize {
        std::thread::available_parallelism().map_or(1, |n| n.get())
    }
}
