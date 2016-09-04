build_subcommand_builder!(
    ViewSubCommandBuilder,
    view,
    "view",
    |builder: SubCommandBuilder<'a>| builder
        .without_author()
        .with_version("0.2.0")
        .with_about("View about")
        .with_usage("usage text")
);

