# rust-image-comparer
 A program to compare large amounts of images in high speed an variable precision.

## Download
 Clone the current repository

## Build
### Requirements
Make sure that you have an up-to-date version of rustup installed.
### Compilation
Change to the directory `image_comparison`
Run:
    cargo build --release
### Result
The result can now be found in `image_comparison/target/release/image_comparison`

## Usage
The following three Arguments have to be passed:
1. The first directory containing images
2. The second directory containing images
3. The requested block size

The images will be processed in the alphabetic order. The block size is variable. A smaller block size results in higher precision, but is also slower.

## To-Dodo
1. Add comments
2. Add documentation (Rust)
3. Add video support
4. Add OpenCL support
5. Build Terminal or Browser UI
