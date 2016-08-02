build_arg_builder!(
    YesArgBuilder,
    yes,
    "y",
    |builder: ArgBuilder<'a>| builder
        .with_long("yes")
        .with_helptext("Positive acknowledge")
        .with_takes_value(false)
        .with_required(false)
        .without_value_name()
);
