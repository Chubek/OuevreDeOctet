use crate::encoder::ohe_lower;
use crate::levenshtein::token_set_ratio;
use nalgebra::DVector;

#[derive(Debug)]
struct Bigram {
    pub word_1: String,
    pub word_2: String,
    pub label_1: u32,
    pub label_2: u32,
    pub word_1_ohe: DVector<f64>,
    pub word_2_ohe: DVector<f64>,
    pub dist: f64,
    pub len_text: usize
}

impl Bigram<T> {
    fn generate_ohe(&mut self) {
        self.word_1_ohe = ohe_lower(self.len_text, self.label_1);
        self.word_2_ohe = ohe_lower(self.len_text, self.label_2);

    }

    fn get_dist(&mut self) {
        self.dist = token_set_ratio(&self.word_1, &self.word_2) / 100f64;
    }
}