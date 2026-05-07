use tikv_jemallocator::Jemalloc;

#[global_allocator]
static ALLOC: Jemalloc = Jemalloc;

pub fn main() {
    println!("Hello world!")
}
