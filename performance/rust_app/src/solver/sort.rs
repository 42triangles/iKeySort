use crate::geom::id_point::IdPoint;
use crate::geom::start_segment::StartEnd;
use rayon::slice::ParallelSliceMut;
use std::time::Instant;
use i_key_sort::sort::two_keys::TwoKeysSort;

pub struct SortSolution;

impl SortSolution {
    pub fn run_segments_sort_unstable<S: StartEnd>(segments: &[S]) {
        let mut data = segments.to_vec();
        let start = Instant::now();

        let n = Self::repeat_count(segments.len());
        for _ in 0..n {
            data.copy_from_slice(&segments);
            data.sort_unstable_by(|s0, s1| s0.cmp_by_start(s1));
        }
        let duration = start.elapsed().as_secs_f64() / (n as f64);

        Self::print_result("sort_unstable", data.last().unwrap().end().x, duration);
    }

    pub fn run_segments_sort_stable<S: StartEnd>(segments: &[S]) {
        let mut data = segments.to_vec();
        let start = Instant::now();

        let n = Self::repeat_count(segments.len());
        for _ in 0..n {
            data.copy_from_slice(&segments);
            data.sort_by(|s0, s1| s0.cmp_by_start(s1));
        }
        let duration = start.elapsed().as_secs_f64() / (n as f64);

        Self::print_result("sort_stable", data.last().unwrap().end().x, duration);
    }

    pub fn run_segments_par_sort_unstable<S: StartEnd>(segments: &[S]) {
        let mut data = segments.to_vec();
        let start = Instant::now();

        let n = Self::repeat_count(segments.len());
        for _ in 0..n {
            data.copy_from_slice(&segments);
            data.par_sort_unstable_by(|s0, s1| s0.cmp_by_start(s1));
        }
        let duration = start.elapsed().as_secs_f64() / (n as f64);

        Self::print_result("par_sort_unstable", data.last().unwrap().end().x, duration);
    }

    pub fn run_segments_par_sort_stable<S: StartEnd>(segments: &[S]) {
        let mut data = segments.to_vec();
        let start = Instant::now();

        let n = Self::repeat_count(segments.len());
        for _ in 0..n {
            data.copy_from_slice(&segments);
            data.par_sort_by(|s0, s1| s0.cmp_by_start(s1));
        }
        let duration = start.elapsed().as_secs_f64() / (n as f64);

        Self::print_result("par_sort_stable", data.last().unwrap().end().x, duration);
    }

    pub fn run_segments_bin_sort<S: StartEnd + Copy + Default>(segments: &[S]) {
        let mut data = segments.to_vec();
        let start = Instant::now();

        let n = Self::repeat_count(segments.len());
        for _ in 0..n {
            data.copy_from_slice(&segments);
            data.sort_by_two_keys(false, |s| s.start().x, |s| s.start().y);
        }
        let duration = start.elapsed().as_secs_f64() / (n as f64);

        Self::print_result("bin_sort", data.last().unwrap().end().x, duration);
    }

    pub fn run_segments_par_bin_sort<S: StartEnd + Copy + Default>(segments: &[S]) {
        let mut data = segments.to_vec();
        let start = Instant::now();

        let n = Self::repeat_count(segments.len());
        for _ in 0..n {
            data.copy_from_slice(&segments);
            data.sort_by_two_keys(true, |s| s.start().x, |s| s.start().y);
        }
        let duration = start.elapsed().as_secs_f64() / (n as f64);

        Self::print_result("par bin_sort", data.last().unwrap().end().x, duration);
    }

    pub fn run_segments_ref_sort<S: StartEnd + Copy + Default>(segments: &[S]) {
        let start = Instant::now();

        let mut rfs: Vec<_> = segments
            .iter()
            .enumerate()
            .map(|(i, s)| IdPoint::new(i, *s.start()))
            .collect();

        rfs.sort_by_two_keys(true, |s| s.start().x, |s| s.start().y);

        let mut data = vec![S::default(); segments.len()];
        for (rf, s) in rfs.iter().zip(segments.iter()) {
            unsafe {
                *data.get_unchecked_mut(rf.id) = *s;
            }
        }
        let duration = start.elapsed().as_secs_f64();

        Self::print_result("ref sort", data.last().unwrap().end().x, duration);
    }

    fn print_result(title: &str, result: i32, duration: f64) {
        println!("{} - {:.6} hash: {}", title, duration, result);
    }

    pub fn run_compare<S: StartEnd + Copy + Default>(segments: &[S]) {
        println!("validation start");
        let mut data_0 = segments.to_vec();
        data_0.sort_by_two_keys(true, |s| s.start().x, |s| s.start().y);
        let mut data_1 = segments.to_vec();
        data_1.sort_by_two_keys(false, |s| s.start().x, |s| s.start().y);
        let mut data_2 = segments.to_vec();
        data_2.par_sort_unstable_by(|s0, s1| s0.cmp_by_start(s1));
        if let Some(index) = compare_by_start(&data_1, &data_2) {
            println!("not valid ser sort index: {}", index);
        }
        if let Some(index) = compare_by_start(&data_0, &data_2) {
            println!("not valid par sort index: {}", index);
        }
    }

    fn repeat_count(len: usize) -> usize {
        (100_000 / len).max(1)
    }

}

fn compare_by_start<S: StartEnd>(data_1: &[S], data_2: &[S]) -> Option<usize> {
    if data_1.len() != data_2.len() {
        return Some(usize::MAX);
    }
    for (i, (s1, s2)) in data_1.iter().zip(data_2.iter()).enumerate() {
        if s1.start() != s2.start() {
            return Some(i);
        }
    }

    None
}
