# iKeySort

[![crates.io version](https://img.shields.io/crates/v/i_key_sort.svg)](https://crates.io/crates/i_key_sort)
[![docs.rs docs](https://docs.rs/i_key_sort/badge.svg)](https://docs.rs/i_key_sort)

A fast sorting algorithm combining bin and counting sort. Optimized for scenarios where a primary key can be extracted to index elements into buckets.

<img src="readme/sort_algorithm.gif" width="512"/>

## Examples

### 1) Sort by a single key

```rust
use i_key_sort::sort::one_key::OneKeySort;

let mut v = vec![5, 1, 4, 1, 3, 2];

v.sort_by_one_key(true, |&x| x);

assert_eq!(v, [1, 1, 2, 3, 4, 5]);
```

`parallel` is ignored unless feature `allow_multithreading` is enabled.

### 2) Sort by a key, then by a comparator

```rust
use i_key_sort::sort::one_key_cmp::OneKeyAndCmpSort;

let mut v = vec![("b", 2), ("a", 3), ("a", 1)];

v.sort_by_one_key_then_by(true, |x| x.0.as_bytes()[0], |a, b| a.1.cmp(&b.1));

assert_eq!(v, [("a", 1), ("a", 3), ("b", 2)]);
```

### 3) Sort by two keys

```rust
use i_key_sort::sort::two_keys::TwoKeysSort;

let mut v = vec![(2, 1), (1, 2), (1, 0)];

v.sort_by_two_keys(true, |x| x.0, |x| x.1);

assert_eq!(v, [(1, 0), (1, 2), (2, 1)]);
```

### 4) Two keys, then comparator (three-way)

```rust
use i_key_sort::sort::two_keys_cmp::TwoKeysAndCmpSort;

let mut v = vec![(1u32, 0i32, 9i32), (1, 0, 3), (1, 1, 1)];

v.sort_by_two_keys_then_by(true, |x| x.0, |x| x.1, |a, b| a.2.cmp(&b.2));

assert_eq!(v, [(1, 0, 3), (1, 0, 9), (1, 1, 1)]);
```


### 5) Reusing a buffer to avoid allocations

```rust
use i_key_sort::sort::two_keys_cmp::TwoKeysAndCmpSort;
use core::mem::MaybeUninit;

let mut buf: Vec<MaybeUninit<i32>> = Vec::new();
let mut v = vec![3, 2, 1];

v.sort_by_one_key_and_buffer(true, &mut buf, |&x| x);

assert_eq!(v, [1, 2, 3]);
```
