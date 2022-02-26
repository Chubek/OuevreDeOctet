use crate::encoder::ohe_lower;
use crate::levenshtein::token_set_ratio;
use nalgebra::DVector;
use crate::utils::rem_punct_split;


#[derive(Debug)]
pub struct Bigram {
    pub word_1: String,
    pub word_2: String,
    pub label_1: u32,
    pub label_2: u32,
    pub word_1_ohe: DVector<f64>,
    pub word_2_ohe: DVector<f64>,
    pub dist: f64,
    pub len_text: usize
}

impl Bigram {
    fn generate_ohe(&mut self) {
        self.word_1_ohe = ohe_lower(self.len_text, self.label_1);
        self.word_2_ohe = ohe_lower(self.len_text, self.label_2);

    }

    fn get_dist(&mut self) {
        self.dist = token_set_ratio(&self.word_1, &self.word_2) as f64 / 100f64;
    }

    pub fn init(&mut self) {
        self.generate_ohe();
        self.get_dist();
    }
}


pub fn new_bigram(word1: String, word2: String, label1: u32, label2: u32, length: usize) -> Bigram {
    let mut bg = Bigram{word_1: word1,
            word_2: word2, label_1: label1,
            label_2: label2, dist: 0f64, len_text: length,
            word_1_ohe: DVector::repeat(1usize, 0.0f64,),
            word_2_ohe: DVector::repeat(1usize, 0.0f64)};
    bg.init();

    return bg

}



pub fn split_text_into_bigram(text: String) -> Vec<Bigram> {
    let text_split = rem_punct_split(text.as_str());

    let mut ret: Vec<Bigram> = Vec::new();

    for (i, word1) in text_split.iter().enumerate() {
        for (j, word2) in text_split.iter().enumerate() {
            if i == j {
                continue
            }

            ret.push(new_bigram(word1.to_string(), word2.to_string(), i as u32, j as u32, text_split.len()))
        }
    }

    return  ret
}