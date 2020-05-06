//! region_folding_mod.rs

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};

use std::fs;
use unwrap::unwrap;
use lazy_static::lazy_static;
use regex::Regex;
use text_size::TextRange;

lazy_static! {
    static ref REGEX_REMOVE_EMAIL: Regex = Regex::new(r#"( <.+?>)"#).unwrap();
}

#[derive(Debug)]
pub struct Fold {
    pub range: TextRange,
    pub kind: FoldKind,
}

pub fn get_vec_of_fold(){
    let code_text = unwrap!(fs::read_to_string("sample_code.txt"));
    // find with regex all start end end region in a vec

}
