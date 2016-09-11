build_subcommand_builder!(
    DeleteSubCommandBuilder,
    delete,
    "delete",
    |builder: SubCommandBuilder<'a>| builder
        .with_author("imag authors, imag@imag-pim.org")
        .with_version("0.2.0")
);

pub use self::delete::*;

