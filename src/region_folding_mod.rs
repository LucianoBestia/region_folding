//! region_folding_mod.rs

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};

use lazy_static::lazy_static;
use regex::{Match, Regex};
use text_size::TextRange;
use text_size::TextSize;
use unwrap::unwrap;

lazy_static! {
    static ref REGEX_REGION: Regex = Regex::new(r#"(?m)^[ \t]*//[ ]?[#]?region:"#).unwrap();
}
lazy_static! {
    static ref REGEX_ENDREGION: Regex = Regex::new(r#"(?m)^[ \t]*//[ ]?[#]?endregion"#).unwrap();
}

#[derive(Debug)]
pub struct Fold {
    pub range: TextRange,
    pub kind: FoldKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FoldKind {
    //Comment,
    //Imports,
    //Mods,
    //Block,
    Region,
}

pub fn get_vec_of_fold(code_text: &str) -> Vec<Fold> {
    let mut vec_fold = vec![];
    let vec_region = get_vec(code_text);
    for region_pos in vec_region {
        vec_fold.push(Fold {
            range: TextRange::new(
                TextSize::from(region_pos.start as u32),
                TextSize::from(region_pos.end as u32),
            ),
            kind: FoldKind::Region,
        });
    }

    //return
    vec_fold
}
#[derive(Debug, PartialEq, Eq)]
enum TagKind {
    Start,
    End,
}
#[derive(Debug, PartialEq, Eq)]
struct RegionTags {
    start: usize,
    tag_kind: TagKind,
}
#[derive(Debug)]
struct RegionPos {
    start: usize,
    end: usize,
}

fn get_vec(code_text: &str) -> Vec<RegionPos> {
    let mut vec_region = Vec::new();
    // find with regex all start end end region in a vec
    let start_regions: Vec<Match> = REGEX_REGION.find_iter(&code_text).collect();
    let end_regions: Vec<Match> = REGEX_ENDREGION.find_iter(&code_text).collect();

    // the end vector goes one-by-one. Here I save the current position
    let mut end_active = 0;
    // the start vector current position can move back and forth.
    let mut start_current = 0;
    // prepare a vector with only starts. They are already sorted
    // and here I prepare the place to insert end regions.
    let mut depth = 0;
    for x in start_regions {
        vec_region.push(RegionPos {
            start: x.start(),
            end: 0,
        });
    }

    loop {
        if start_current >= vec_region.len() {
            //println!("break because start_current >= vec_region.len() {} {}",start_current,vec_region.len());
            break;
        } else if end_active >= end_regions.len() {
            //println!("break because end_active >= end_regions.len() {} {}",end_active,end_regions.len());
            break;
        } else if vec_region[start_current].end != 0 {
            // jump over regions that have end
            start_current += 1;
        //println!("jump over regions that have end: start_current += 1; {}",start_current);
        } else if start_current + 1 >= vec_region.len()
            || vec_region[start_current + 1].end != 0
            || vec_region[start_current + 1].start > end_regions[end_active].start()
        {
            if vec_region[start_current].start < end_regions[end_active].start() {
                vec_region[start_current].end = end_regions[end_active].start();
                //println!("start   end: {} {}",print_to_end_of_line(&code_text, vec_region[start_current].start)print_to_end_of_line(&code_text, vec_region[start_current].end),);

                end_active += 1;
                //println!("end_active += 1; {}", end_active);
                // the start_current can go up or down, it depends on the depth.
                if depth > 0 {
                    //println!("depth > 0; {} {}", depth, start_current);
                    start_current -= 1;
                    depth -= 1;
                //println!("after; {} {}", depth, start_current);
                } else {
                    start_current += 1;
                    //println!("depth = 0; start_current = {}", start_current);
                }
            } else {
                end_active += 1;
            }
        } else {
            start_current += 1;
            depth += 1;
            //println!("depth += 1; {} {}", depth, start_current);
        }
    }
    //remove the start region without end region
    vec_region.retain(|x| x.end != 0);

    //println!("{:?}", vec_region);

    //return
    vec_region
}

pub fn print_to_end_of_line(code_text: &str, pos: usize) -> &str {
    if let Some(pos_end_line) = code_text[pos..].find('\n') {
        &code_text[pos..pos + pos_end_line]
    } else {
        //until the end of file
        &code_text[pos..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_vec_of_fold_for_file_sample1_code() {
        let vec_of_fold = get_vec_of_fold("sample1_code.txt");
        let result ="[Fold { range: 0..102, kind: Region }, Fold { range: 53..76, kind: Region }, Fold { range: 442..464, kind: Region }, Fold { range: 3580..3884, kind: Region }, Fold { range: 4759..5128, kind: Region }]";
        assert_eq!(format!("{:?}", vec_of_fold), result);
    }
}
