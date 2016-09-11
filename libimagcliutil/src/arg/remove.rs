build_arg_builder!(
    RemoveArgBuilder,
    remove,
    "remove",
    |builder: ArgBuilder<'a>| builder
        .with_short("r")
        .with_long("remove")
        .with_takes_value(true)
        .with_required(false)
);

pub use self::remove::*;

