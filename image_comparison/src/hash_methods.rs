use ndarray;
use ndarray::Axis;

pub fn calc(block: &ndarray::Array<u8, ndarray::Ix3>) -> [f64; 3] {
    let mut values: [u32; 3] = [0, 0, 0];
    let mut counter = 0.0;
    for line in block.axis_iter(Axis(0)) {
        for pixel in line.axis_iter(Axis(1)) {
            values[0] += pixel[0] as u32;
            values[1] += pixel[1] as u32;
            values[2] += pixel[2] as u32;
            counter += 1.0;
        }
    }
    let values: [f64; 3] = [values[0] as f64 / counter, values[1] as f64 / counter, values[2] as f64 / counter];
    return values;
}