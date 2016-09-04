build_subcommand_builder!(
    AddSubCommandBuilder,
    add,
    "add",
    |builder: SubCommandBuilder<'a>| builder
        .with_author("imag authors, imag@imag-pim.org")
        .with_version("0.2.0")
        .with_about("Add about")
        .with_usage("usage text")
);

