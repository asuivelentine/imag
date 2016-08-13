build_subcommand_builder!(
    AddSubCommandBuilder,
    add,
    "add",
    |builder: SubCommandBuilder<'a>| builder
        .without_author()
        .with_version("0.1")
        .with_about("Add about")
        .with_usage("usage text")
);

