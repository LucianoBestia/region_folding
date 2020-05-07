# region_folding  

[comment]: # (lmake_readme cargo.toml data start)
version: 2020.507.1558  date: 2020-05-07 authors: Luciano Bestia  
**proof of concept for region folding in rust-analyzer and VSCode**

[comment]: # (lmake_readme cargo.toml data end)

## VSCode and rust-analyzer

I would like to have region_folding in VSCode for rust.  
Rust-analyzer already prepares some Fold structs for folding:
`rust-analyzer\crates\ra_ide\src\folding_ranges.rs`
`fn folding_ranges(file: &SourceFile) -> Vec<Fold>`

Maybe it would be possible to add also region_folding.
But touching a complex project like rust-analyzer is not easy to start.
I will make a CLI just as proof-of-concept.
Maybe somebody more experienced could add this to rust-analyzer.

List of prepared make tasks for development: build, run, doc, publish,...  
`clear; cargo make test`  
`clear; cargo make release`  
`clear; cargo make run_rel1`  
`clear; cargo make run_rel2`  
`clear; cargo make run_rel3`  
`clear; cargo make run_rel4`  
`clear; cargo make run_rel5`  
`clear; cargo make run_rel6`  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://bestia.dev/cargo_crev_web/query/num-traits>  
