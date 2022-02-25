use nalgebra::{Dynamic, max, DVector};


pub fn ohe_lower(max_len: usize, label_one: u32) -> DVector<f64> {
    let mut ret = DVector::from_element(max_len, 0f64);
    ret[label_one as usize] = 1.0f64;

    return ret
}