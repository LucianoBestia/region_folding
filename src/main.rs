// region: lmake_readme include "readme.md" //! A
//! # lmake_semver  
//!
//! version: 0.1.6  date: 2020-04-24 authors: Luciano Bestia  
//! **Increments the patch or minor version in cargo.toml.**
//!
//!
//! ## Install
//!
//! `cargo install lmake_semver`  
//!
//! ## Run
//!
//! Run it with this arguments:  
//!
//! `lmake_semver --increment=patch`  
//! `lmake_semver --increment=minor`  
//!
//! ## Development
//!
//! List of prepared make tasks for development: build, run, doc, publish,...  
//! `clear; cargo make`  
//!
//! ## Tasks in Makefile.toml
//!
//! Libraries use semver. On every build release you can increment patch.  
//!
//! ```toml
//! [tasks.release]
//! description = "cargo build release"
//! clear = true
//! dependencies = [
//!     "semver_increment_patch",
//!     "build_release",
//! ]
//!
//! [tasks.semver_increment_patch]
//! clear = true
//! private = true
//! description = "increment semver patch"
//! script= ["lmake_semver --increment=patch"]
//! ```
// endregion: lmake_readme include "readme.md" //! A

// region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    // variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,

)]
#![allow(
    // library from dependencies have this clippy warnings. Not my code.
    // Why is this bad: It will be more difficult for users to discover the purpose of the crate, 
    // and key information related to it.
    clippy::cargo_common_metadata,
    // Why is this bad : This bloats the size of targets, and can lead to confusing error messages when 
    // structs or traits are used interchangeably between different versions of a crate.
    clippy::multiple_crate_versions,
    // Why is this bad : As the edition guide says, it is highly unlikely that you work with any possible 
    // version of your dependency, and wildcard dependencies would cause unnecessary 
    // breakage in the ecosystem.
    clippy::wildcard_dependencies,
    // Rust is more idiomatic without return statement
    // Why is this bad : Actually omitting the return keyword is idiomatic Rust code. 
    // Programmers coming from other languages might prefer the expressiveness of return. 
    // It’s possible to miss the last returning statement because the only difference 
    // is a missing ;. Especially in bigger code with multiple return paths having a 
    // return keyword makes it easier to find the corresponding statements.
    clippy::implicit_return,
    // I have private function inside a function. Self does not work there.
    // Why is this bad: Unnecessary repetition. Mixed use of Self and struct name feels inconsistent.
    clippy::use_self,
    // Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    // because then wasm-pack build --target web returns an error: export run not found 
    // Why is this bad: In general, it is not. Functions can be inlined across crates when that’s profitable 
    // as long as any form of LTO is used. When LTO is disabled, functions that are not #[inline] 
    // cannot be inlined across crates. Certain types of crates might intend for most of the 
    // methods in their public API to be able to be inlined across crates even when LTO is disabled. 
    // For these types of crates, enabling this lint might make sense. It allows the crate to 
    // require all exported methods to be #[inline] by default, and then opt out for specific 
    // methods where this might not make sense.
    clippy::missing_inline_in_public_items,
    // Why is this bad: This is only checked against overflow in debug builds. In some applications one wants explicitly checked, wrapping or saturating arithmetic.
    // clippy::integer_arithmetic,
    // Why is this bad: For some embedded systems or kernel development, it can be useful to rule out floating-point numbers.
    clippy::float_arithmetic,
    // Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
    // Why is this bad : Splitting the implementation of a type makes the code harder to navigate.
    clippy::multiple_inherent_impl,

    clippy::missing_docs_in_private_items,
)]
// endregion

// region: mod, extern and use statements
mod region_folding_mod;

//use clap::*;
use unwrap::unwrap;

#[allow(unused_imports)]
use ansi_term::Colour::{Green, Red, Yellow};
use clap::{App, Arg};
use std::env;
use std::fs;
// endregion

#[allow(clippy::print_stdout, clippy::integer_arithmetic)]
/// The program starts here.
fn main() {
    // this function is different for Windows and for Linux.
    // Look at the code of this function (2 variations).
    enable_ansi_support();

    // define the CLI input line parameters using the clap library
    let arguments = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("file_name")
                .required(true)
                .value_name("file_name")
                .help("sample1_code.txt"),
        )
        .get_matches();

    if let Some(file_name) = arguments.value_of("file_name") {
        let code_text = unwrap!(fs::read_to_string(file_name));
        let vec_of_fold = region_folding_mod::get_vec_of_fold(&code_text);
        //println!("{:#?}", vec_of_fold);
        use crate::region_folding_mod::print_to_end_of_line;
        for fold in vec_of_fold {
            println!(
                "{:05} {:05} :  {}   {}",
                usize::from(fold.range.start()),
                usize::from(fold.range.end()),
                print_to_end_of_line(&code_text, fold.range.start().into()).trim(),
                print_to_end_of_line(&code_text, fold.range.end().into()).trim(),
            );
        }
    }
}

// region: different function code for Linux and Windows
#[cfg(target_family = "windows")]
/// only on windows "enable ansi support" must be called
pub fn enable_ansi_support() {
    let _enabled = ansi_term::enable_ansi_support();
}

#[cfg(target_family = "unix")]
//on Linux "enable ansi support" must not be called
pub fn enable_ansi_support() {
    // do nothing
}
// endregion
