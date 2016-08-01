#![deny(
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_mut,
    unused_qualifications,
    while_true,
)]

extern crate clap;

pub mod argbuilder;
pub mod name;
pub mod create;
pub mod delete;
pub mod edit;
pub mod add;
pub mod noedit;
pub mod remove;
pub mod range;
pub mod timerange;
pub mod daterange;
pub mod select;
pub mod yes;

