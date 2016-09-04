build_subcommand_builder!(
    ListSubCommandBuilder,
    list,
    "list",
    |builder: SubCommandBuilder<'a>| builder
        .without_author()
        .with_version("0.2.0")
        .with_about("List about")
        .with_usage("usage text")
);

