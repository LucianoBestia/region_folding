//! region_folding_mod.rs

use lazy_static::lazy_static;
use regex::{Match, Regex};
use text_size::{TextRange, TextSize};
use unwrap::unwrap;

// find this lines with regex:
// (?m) multi-line regex
// ^[ \t]* from start of line zero or any number of space or tab
// the comment sign for rust //
// [ ]? zero or one space
// [#]? zero or one #
// "region:"  or "endregion"
lazy_static! {
    static ref REGEX_REGION: Regex = unwrap!(Regex::new(r#"(?m)^[ \t]*//[ ]?[#]?region:"#));
}
lazy_static! {
    static ref REGEX_ENDREGION: Regex = unwrap!(Regex::new(r#"(?m)^[ \t]*//[ ]?[#]?endregion"#));
}

/// struct from rust-analyzer
#[derive(Debug)]
pub struct Fold {
    pub range: TextRange,
    pub kind: FoldKind,
}
/// struct from rust-analyzer. Added Region
#[derive(Debug, PartialEq, Eq)]
pub enum FoldKind {
    //Comment,
    //Imports,
    //Mods,
    //Block,
    Region,
}
// temporary struct for easier work
#[derive(Debug)]
struct RegionPos {
    start: usize,
    end: usize,
}

/// return the Fold vector for Folding regions in rust language
pub fn get_vec_of_fold(code_text: &str) -> Vec<Fold> {
    let mut vec_fold = vec![];
    let vec_region = get_vec(code_text);
    // convert the easy vector to fold vector
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

/// find regex for region and endregion
/// combine pairs of region and endregion (also nested)
fn get_vec(code_text: &str) -> Vec<RegionPos> {
    let mut vec_region = Vec::new();
    // find with regex all start end end region in a vec
    let start_regions: Vec<Match> = REGEX_REGION.find_iter(&code_text).collect();
    let end_regions: Vec<Match> = REGEX_ENDREGION.find_iter(&code_text).collect();

    // the end vector goes one-by-one. Here I save the current position
    let mut end_active = 0;
    // the start vector current position can move back and forth because of nested
    let mut start_current = 0;
    // depth of nested pairs to go backward
    let mut depth = 0;
    // prepare a vector with only starts. They are already sorted
    for start_region in start_regions {
        vec_region.push(RegionPos {
            start: start_region.start(),
            end: 0,
        });
    }

    while start_current < vec_region.len() && end_active < end_regions.len() {
        if vec_region[start_current].end != 0 {
            // jump over regions that have end already inserted
            start_current += 1;
        //println!("jump over regions that have end: start_current += 1; {}",start_current);
        } else if start_current + 1 >= vec_region.len()
            || vec_region[start_current + 1].end != 0
            || vec_region[start_current + 1].start > end_regions[end_active].start()
        {
            if vec_region[start_current].start < end_regions[end_active].start() {
                // this is a pair. Insert end into vector
                vec_region[start_current].end = end_regions[end_active].start();
                //println!("start   end: {} {}",get_str_to_end_of_line(&code_text, vec_region[start_current].start)get_str_to_end_of_line(&code_text, vec_region[start_current].end),);
                // end regions can go only up
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
                // end regions can go only up
                end_active += 1;
            }
        } else {
            //nested regions
            start_current += 1;
            depth += 1;
            //println!("depth += 1; {} {}", depth, start_current);
        }
    }
    //remove the regions without end region
    vec_region.retain(|x| x.end != 0);
    //println!("{:?}", vec_region);

    //return
    vec_region
}

/// get str to end of line
pub fn get_str_to_end_of_line(code_text: &str, pos: usize) -> &str {
    if let Some(pos_end_line) = code_text[pos..].find('\n') {
        &code_text[pos..pos + pos_end_line]
    } else {
        //until the end of file
        &code_text[pos..]
    }
}

/// print nice for fold vector debug
pub fn print_vec(vec_of_fold: Vec<Fold>, code_text: &str) -> String {
    let mut ret_str = String::new();
    for fold in vec_of_fold {
        ret_str.push_str(&format!(
            "{:5} {:5} :  {:14}   {:20}\n",
            usize::from(fold.range.start()),
            usize::from(fold.range.end()),
            get_str_to_end_of_line(code_text, fold.range.start().into()).trim(),
            get_str_to_end_of_line(code_text, fold.range.end().into()).trim(),
        ));
    }
    //return
    ret_str
}

// unit testing
// integration testing doesn't work nice for binary projects (main)
#[cfg(test)]
mod test {
    use super::*;

    use std::fs;
    /// test empty code
    #[test]
    fn test_empty_code() {
        let code_text = "";
        let vec_of_fold = get_vec_of_fold(code_text);
        let output = print_vec(vec_of_fold, code_text);
        let result = "";
        assert_eq!(output, result);
    }

    /// test with sample1
    #[test]
    fn test_get_vec_of_fold_for_file_sample1_code() {
        let code_text = unwrap!(fs::read_to_string("samples/sample1_no_regions.txt"));
        let vec_of_fold = get_vec_of_fold(&code_text);
        let output = print_vec(vec_of_fold, &code_text);
        let result = "";
        assert_eq!(output, result);
    }
    /// test with sample2
    #[test]
    fn test_get_vec_of_fold_for_file_sample2_code() {
        let code_text = unwrap!(fs::read_to_string("samples/sample2_not_nested.txt"));
        let vec_of_fold = get_vec_of_fold(&code_text);
        let output = print_vec(vec_of_fold, &code_text);
        let result = "  220   260 :  // region: 1     // endregion: 1     
  300   340 :  //region: 2      //endregion: 2      
  380   420 :  //#region: 3     //#endregion: 3     
  460   500 :  // #region: 4    // #endregion: 4    
  540   580 :  // region: 5     // endregion: 5     
";
        assert_eq!(output, result);
    }
    /// test with sample3
    #[test]
    fn test_get_vec_of_fold_for_file_sample3_code() {
        let code_text = unwrap!(fs::read_to_string("samples/sample3_one_nested.txt"));
        let vec_of_fold = get_vec_of_fold(&code_text);
        let output = print_vec(vec_of_fold, &code_text);
        let result = "   60   180 :  // region: 1     // endregion: 1     
  100   140 :  // #region: 2    // endregion: 2     
";
        assert_eq!(output, result);
    }
    /// test with sample4
    #[test]
    fn test_get_vec_of_fold_for_file_sample4_code() {
        let code_text = unwrap!(fs::read_to_string("samples/sample4_three_nested.txt"));
        let vec_of_fold = get_vec_of_fold(&code_text);
        let output = print_vec(vec_of_fold, &code_text);
        let result = "   60   520 :  // region: 1     // endregion: 1     
  100   480 :  // #region: 2    // endregion: 2     
  140   420 :  // #region: 3    // endregion: 3     
  220   340 :  // #region: 4    // endregion: 4     
";
        assert_eq!(output, result);
    }
    /// test with sample5
    #[test]
    fn test_get_vec_of_fold_for_file_sample5_code() {
        let code_text = unwrap!(fs::read_to_string("samples/sample5_no_ending.txt"));
        let vec_of_fold = get_vec_of_fold(&code_text);
        let output = print_vec(vec_of_fold, &code_text);
        let result = "  140   180 :  // region: 4     // endregion: 4     
";
        assert_eq!(output, result);
    }
    /// test with sample6
    #[test]
    fn test_get_vec_of_fold_for_file_sample6_code() {
        let code_text = unwrap!(fs::read_to_string("samples/sample6_no_start.txt"));
        let vec_of_fold = get_vec_of_fold(&code_text);
        let output = print_vec(vec_of_fold, &code_text);
        let result = "  120   160 :  // region: 4     // endregion: 4     
";
        assert_eq!(output, result);
    }
}
