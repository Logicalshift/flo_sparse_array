//! # flo_sparse_array
//! 
//! ```
//! use flo_sparse_array::*;
//! 
//! let mut sparse_array = SparseArray::empty();
//! sparse_array.insert(12345, "Test");
//! assert!(sparse_array.get(12345) == Some(&"Test"));
//! ```
//! 
//! This crate provides an implementation of a simple sparse array type. This is a specialised type of hash table that
//! maps from a `usize` to any data type. Its main advantage over a normal Rust `HashMap` is performance when reading
//! or updating existing values. This performance is very consistent too.
//! 
//! It's also usually slightly faster when writing new values but as both this and the main hash functions have a random 
//! element to their performance the difference here can vary slightly based on chance.
//! 
//! The hash algorithm used to implement this sparse array type is the Cuckoo hash algorithm: this can access any location
//! with only two memory accesses after calculating the hash value.

mod sparse_array;

pub use sparse_array::*;
