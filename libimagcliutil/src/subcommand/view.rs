build_subcommand_builder!(
    ViewSubCommandBuilder,
    view,
    "view",
    |builder: SubCommandBuilder<'a>| builder
        .with_author("imag authors, imag@imag-pim.org")
        .with_version("0.2.0")
);

pub use self::view::*;

