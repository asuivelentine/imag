build_arg_builder!(
    NoEditArgBuilder,
    noedit,
    "no-edit",
    |builder: ArgBuilder<'a>| builder
        .with_short("n")
        .with_long("no-edit")
        .with_helptext("do not edit the given element")
        .with_takes_value(false)
        .with_required(false)
        .with_value_name("NOEDIT")
);

pub use self::noedit::*;

