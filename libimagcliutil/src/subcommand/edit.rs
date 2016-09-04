build_subcommand_builder!(
    EditSubCommandBuilder,
    edit,
    "edit",
    |builder: SubCommandBuilder<'a>| builder
        .without_author()
        .with_version("0.2.0")
        .with_about("Edit about")
        .with_usage("usage text")
);

