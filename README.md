# rust-image-comparer
 A program to compare large amounts of images in high speed an variable precision.

## Download
 Clone the current repository

## Build
### Requirements
Make sure that you have an up-to-date version of rustup installed.
### Compilation
Change to the directory 'image_comparison'
Run:
   cargo build --release
### Result
The result can now be found in 'image_comparison/target/release/image_comparison

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

### License
Copyright 2020 Hannes Furmans

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.