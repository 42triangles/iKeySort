#[cfg(feature = "allow_multithreading")]
pub mod parallel;
pub mod serial;
pub mod bin_layout;
pub mod key;
mod mapper;
mod spread;
mod min_max;
mod log2;