build_arg_builder!(
    YesArgBuilder,
    yes,
    "y",
    |builder: ArgBuilder<'a>| builder
        .with_long("yes")
        .with_helptext("Don't ask for confirmation")
        .with_takes_value(false)
        .with_required(false)
);

pub use self::yes::*;

