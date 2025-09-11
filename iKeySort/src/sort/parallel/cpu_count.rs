pub(super) struct CPUCount;

impl CPUCount {
    #[inline]
    pub(super) fn count() -> usize {
        std::thread::available_parallelism().map_or(1, |n| n.get())
    }

    #[cfg(not(debug_assertions))]
    #[inline(always)]
    pub(super) fn should_parallel(len: usize) -> Option<usize> {
        const MIN_LEN_PER_TASK: usize = 32_000;
        let cpu = CPUCount::count().max(1);
        if len >= cpu * MIN_LEN_PER_TASK {
            Some(cpu)
        } else {
            None
        }
    }

    #[cfg(debug_assertions)]
    #[inline(always)]
    pub(super) fn should_parallel(_: usize) -> Option<usize> {
        // Some(CPUCount::count().max(1))
        Some(1)
    }
}
