//! # Amyc
//!
//! This is the main driver program of the rewritten Amyc. It connects the moving pieces in a
//! pipeline to drive compilation of Amy into WebAssembly. The concept of a session is used to
//! store certain global state (per compilation) when necessary.

use tikv_jemallocator::Jemalloc;

#[global_allocator]
static ALLOC: Jemalloc = Jemalloc;

pub fn main() {
    println!("Hello world!")
}
