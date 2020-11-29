use ndarray;
use ndarray::Axis;
use rayon::prelude::*;
use std::fs::read_dir;
use std::path::PathBuf;

pub struct Image {
    image_data: ndarray::Array3<u8>,
    blocks: Vec<ndarray::Array<u8, ndarray::Ix3>>,
    pub hash_map: Vec<[f64; 3]>,
    pub path: PathBuf,
}

impl Image {
    pub fn calc_blocks(&mut self, b_size: usize) {
        let mut blocks = Vec::new();
        let x_arr = self.image_data.axis_chunks_iter(Axis(0), b_size);
        for y in x_arr {
            let y_arr = y.axis_chunks_iter(Axis(1), 10);
            for x in y_arr {
                blocks.push(x.to_owned());
            }
        }
        self.blocks = blocks;
    }
    pub fn calc_hashmap(&mut self, f: fn(&ndarray::Array<u8, ndarray::Ix3>) -> [f64; 3]) {
        let res: Vec<[f64; 3]> = self.blocks.par_iter().map(|block| f(block)).collect();
        self.hash_map = res;
    }
}

pub fn load_image(p: PathBuf) -> Image {
    let arr = match ndarray_image::open_image(&p, ndarray_image::Colors::Rgb) {
        Ok(data) => data,
        Err(error) => panic!("{}", error)
    };
    let blocks = Vec::new();
    let hash_map = Vec::new();
    let res = Image {
        image_data: arr,
        blocks: blocks,
        hash_map: hash_map,
        path: p.to_owned(),
    };
    return res;
}

pub fn load_dir(p: &str) -> Vec<PathBuf> {
    let dir = match read_dir(p) {
        Ok(data) => data,
        Err(e) => panic!("{}", e),
    };
    let mut ret_images = Vec::new();
    for d in dir {
        let tmp = match d {
            Ok(data) => data,
            Err(e) => panic!("{}", e),
        };
        ret_images.push(tmp.path());
    }
    ret_images.sort();
    return ret_images;
}