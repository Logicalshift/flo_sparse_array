# flo_sparse_array

```Rust
use flo_sparse_array::*;

let mut sparse_array = SparseArray::empty();
sparse_array.insert(12345, "Test");
assert!(sparse_array.get(12345) == "Test");
```

This crate provides an implementation of a simple sparse array type. This is a specialised type of hash table that
maps from a `usize` to any data type. Its main advantage over a normal Rust `HashMap` is performance when reading
or updating existing values. This performance is very consistent too.

It's also usually slightly faster when writing new values but as both this and the main hash functions have a random 
element to their performance the difference here can vary slightly based on chance.

The hash algorithm used to implement this sparse array type is the Cuckoo hash algorithm: this can access any location
with only two memory accesses after calculating the hash value.

# Benchmark comparison to the HaspMap type

Rust 1.70.0

```
Benchmarking store_hashmap_100k_to_self: Collecting 100 samples in estimated 5.6
store_hashmap_100k_to_self
                        time:   [7.0570 ms 7.0644 ms 7.0729 ms]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

Benchmarking store_sparse_array_100k_to_self: Collecting 100 samples in estimate
store_sparse_array_100k_to_self
                        time:   [5.0491 ms 5.1826 ms 5.3174 ms]

Benchmarking fetch_hashmap_100k: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 8.5s, enable flat sampling, or reduce sample count to 50.
Benchmarking fetch_hashmap_100k: Collecting 100 samples in estimated 8.4986 s (5
fetch_hashmap_100k      time:   [1.6831 ms 1.6845 ms 1.6859 ms]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

Benchmarking fetch_sparse_array_100k: Collecting 100 samples in estimated 5.5865
fetch_sparse_array_100k time:   [269.83 µs 270.08 µs 270.38 µs]
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low severe
  7 (7.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe

Benchmarking insert_overwrite_hashmap_100k: Collecting 100 samples in estimated
insert_overwrite_hashmap_100k
                        time:   [2.5645 ms 2.8273 ms 3.0931 ms]

Benchmarking insert_overwrite_sparse_array_100k: Collecting 100 samples in estim
insert_overwrite_sparse_array_100k
                        time:   [536.66 µs 538.18 µs 539.75 µs]

Benchmarking update_hashmap_100k: Collecting 100 samples in estimated 5.1938 s (
update_hashmap_100k     time:   [2.9770 ms 3.2273 ms 3.4757 ms]

Benchmarking insert_overwrite_sparse_array_100k #2: Collecting 100 samples in es
insert_overwrite_sparse_array_100k #2
                        time:   [258.76 µs 259.54 µs 260.43 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
```
