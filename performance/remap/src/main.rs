use std::mem::MaybeUninit;
use i_key_sort::sort::one_key::OneKeySort;

#[derive(Clone, Copy, Default)]
struct LineRange {
    min: i32,
    max: i32
}

#[derive(Clone, Copy)]
struct XDig {
    x: i32,
    range: LineRange
}

struct YDig {
    y: i32,
    range: LineRange
}


fn main() {

    let mut x_vec =vec![
        XDig { x: 6, range: Default::default() },
        XDig { x: 3, range: Default::default() },
        XDig { x: 8, range: Default::default() },
        XDig { x: 2, range: Default::default() }
    ];

    let mut y_vec =vec![
        YDig { y: 2, range: Default::default() },
        YDig { y: 5, range: Default::default() },
        YDig { y: 1, range: Default::default() },
        YDig { y: 3, range: Default::default() }
    ];

    let mut x_buf = Vec::new();
    x_vec.sort_by_one_key_and_buffer(false, &mut x_buf, |d| d.x);

    let y_buf = unsafe { core::slice::from_raw_parts_mut(
        x_buf.as_mut_ptr() as *mut MaybeUninit<YDig>,
        x_buf.len(),
    )};

    y_vec.sort_by_one_key_and_buffer(false, y_buf, |d| d.y);

    println!("Hello, world!");
}
