#![feature(core_intrinsics)]



use std::env;

mod loader;
mod hash_methods;
mod comparer;

/// Image Comparison - compare huge chunks of images
/// Hannes Furmans -- Open Source

/// Main Function. Reading args and starting the comparison.
/// 1. Argument => first directory
/// 2. Argument => second directory
/// 3. Argument => block size (comparison resolution)

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let dir1 = &args[1];
    let dir2 = &args[2];
    let b_size = match args[3].parse::<usize>() {
        Ok(data) => data,
        Err(e) => panic!("{}", e)
    };
    println!("{}", b_size);
    comparer::compare(dir1, dir2, b_size);
}