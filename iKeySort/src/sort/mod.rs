#[cfg(feature = "allow_multithreading")]
mod parallel;
mod serial;
mod bin_layout;
mod log2;
mod mapper;
mod min_max;
mod spread;
pub mod key;
pub mod key_sort;