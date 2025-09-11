use std::f64::consts::PI;
use crate::geom::id_segment::IdSegment;
use crate::geom::index_segm::IndexSegment;
use crate::geom::point::Point;
use crate::geom::segm::Segment;
use crate::solver::sort::SortSolution;

pub struct CircleTest {
    segments: Vec<Segment>,
    index_segments: Vec<IndexSegment>,
    id_segments: Vec<IdSegment>,
}

impl CircleTest {

    pub fn new(n: usize) -> Self {
        let segments = Self::circle_segments(100_000, n);
        let index_segments: Vec<_> = segments.iter().enumerate().map(|(i, s)|IndexSegment::new(i, s)).collect();
        let id_segments: Vec<_> = segments.iter().enumerate().map(|(i, s)|IdSegment::new(i, s)).collect();
        Self {
            segments,
            index_segments,
            id_segments,
        }
    }

    pub fn run_all(&self) {
        println!("Circle");
        self.run_segments();
        println!();
        self.run_index_segments();
        println!();
        self.run_id_segments();
    }

    pub fn run_segments(&self) {
        println!("Segments test n = {}", self.segments.len());
        println!();
        SortSolution::run_segments_sort_stable(&self.segments);
        SortSolution::run_segments_sort_unstable(&self.segments);
        SortSolution::run_segments_par_sort_stable(&self.segments);
        SortSolution::run_segments_par_sort_unstable(&self.segments);
        SortSolution::run_segments_bin_sort(&self.segments);
        SortSolution::run_segments_par_bin_sort(&self.segments);
        SortSolution::run_segments_ref_sort(&self.segments);
        SortSolution::run_compare(&self.segments);
    }

    pub fn run_index_segments(&self) {
        println!("Index Segments test n = {}", self.index_segments.len());
        println!();
        SortSolution::run_segments_sort_stable(&self.index_segments);
        SortSolution::run_segments_sort_unstable(&self.index_segments);
        SortSolution::run_segments_par_sort_stable(&self.index_segments);
        SortSolution::run_segments_par_sort_unstable(&self.index_segments);
        SortSolution::run_segments_bin_sort(&self.index_segments);
        SortSolution::run_segments_par_bin_sort(&self.index_segments);
        SortSolution::run_segments_ref_sort(&self.index_segments);
    }

    pub fn run_id_segments(&self) {
        println!("Id Segments test n = {}", self.id_segments.len());
        println!();
        SortSolution::run_segments_sort_stable(&self.id_segments);
        SortSolution::run_segments_sort_unstable(&self.id_segments);
        SortSolution::run_segments_par_sort_stable(&self.id_segments);
        SortSolution::run_segments_par_sort_unstable(&self.id_segments);
        SortSolution::run_segments_bin_sort(&self.id_segments);
        SortSolution::run_segments_par_bin_sort(&self.id_segments);
        SortSolution::run_segments_ref_sort(&self.id_segments);
    }

    fn circle_segments(radius: i32, n: usize) -> Vec<Segment> {
        let mut vec = Vec::with_capacity(n);
        let da = 2.0 * PI / n as f64;
        let mut angle = da;
        let start = Point::new(radius, 0);
        let r = radius as f64;
        let mut p0 = start;
        for _ in 0..n {
            let (sn, cs) = angle.sin_cos();
            let x = (cs * r) as i32;
            let y = (sn * r) as i32;

            let p1 = Point::new(x, y);

            vec.push(Segment::new(p0, p1));

            p0 = p1;
            angle += da;
        }

        vec
    }
}
