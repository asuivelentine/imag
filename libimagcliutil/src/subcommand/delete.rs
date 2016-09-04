build_subcommand_builder!(
    DeleteSubCommandBuilder,
    delete,
    "delete",
    |builder: SubCommandBuilder<'a>| builder
        .with_author("imag authors, imag@imag-pim.org")
        .with_version("0.2.0")
        .with_about("Delete about")
        .with_usage("usage text")
);

