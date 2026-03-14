use rayon::ThreadPool;
use rayon::ThreadPoolBuilder;

pub fn build_pool(threads: u32) -> ThreadPool {
    ThreadPoolBuilder::new()
        .num_threads(threads as usize)
        .build()
        .expect("Failed to build thread pool")
}
