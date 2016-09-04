build_subcommand_builder!(
    ViewSubCommandBuilder,
    view,
    "view",
    |builder: SubCommandBuilder<'a>| builder
        .without_author()
        .with_version("0.1")
        .with_about("View about")
        .with_usage("usage text")
);

pub use self::view::*;

