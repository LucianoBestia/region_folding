# region_folding  

[comment]: # (lmake_readme cargo.toml data start)

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