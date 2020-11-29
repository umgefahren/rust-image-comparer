use crate::loader;
use crate::hash_methods;
use std::time::Instant;

pub fn compare(dir1: &str, dir2: &str, bsize: usize) {
    let mut images1 = loader::load_dir(dir1);
    let mut images2 = loader::load_dir(dir2);
    for (image1_path, image2_path) in images1.iter_mut().zip(images2.iter_mut()) {
        let start = Instant::now();
        let mut image1 = loader::load_image(image1_path.to_owned());
        let mut image2 = loader::load_image(image2_path.to_owned());
        image1.calc_blocks(bsize);
        image2.calc_blocks(bsize);
        let path_image1 = match &mut image1_path.file_name() {
            Some(data) => match data.to_str() {
                Some(data) => data.to_owned(),
                _ => panic!("Help")
            },
            _ => panic!("Help")
        };
        let path_image2 = match &mut image2_path.file_name() {
            Some(data) => match data.to_str() {
                Some(data) => data.to_owned(),
                _ => panic!("Help")
            },
            _ => panic!("Help")
        };
        print!("Calculating alpha for {} and {} 🔄", path_image1, path_image2);
        image1.calc_hashmap(hash_methods::calc);
        image2.calc_hashmap(hash_methods::calc);
        print!("\rCalculation complete for {} and {} ✅ {} ns\n", path_image1, path_image2, start.elapsed().as_nanos());
        if image1.hash_map == image2.hash_map {
            println!("Images seem equal at current resolution.")
        } else {
            let mut counter = 0;
            let mut alpha = [0.0, 0.0, 0.0];
            for (image1_m_item, image2_m_item) in image1.hash_map.iter().zip(image2.hash_map.iter()) {
                alpha[0] += (image1_m_item[0] - image2_m_item[0]).abs();
                alpha[1] += (image1_m_item[1] - image2_m_item[1]).abs();
                alpha[2] += (image1_m_item[2] - image2_m_item[2]).abs();
                counter += 1;
            }
            let alpha = [alpha[0] / counter as f64, alpha[1] / counter as f64, alpha[2] / counter as f64];
            println!("Average alpha: r = {} g = {} b = {}", alpha[0], alpha[1], alpha[2]);
        }
    }
}