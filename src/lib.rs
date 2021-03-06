//! HAMT implementation whose sub-trees can be shared over threads.
//!
//! Hash-Array Mapped Trie (HAMT) is a data structure popular as a map (a.k.a.
//! associative array or dictionary) or set. Its immutable variant is adopted
//! widely by functional programming languages like Scala and Clojure to
//! implement immutable and memory-efficient associative arrays and sets.

mod bitmap;
mod bucket;
mod hamt;
mod map;
mod node;

pub use map::Map;
