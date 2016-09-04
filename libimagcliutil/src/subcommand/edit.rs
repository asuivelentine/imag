build_subcommand_builder!(
    EditSubCommandBuilder,
    edit,
    "edit",
    |builder: SubCommandBuilder<'a>| builder
        .with_author("imag authors, imag@imag-pim.org")
        .with_version("0.2.0")
);

