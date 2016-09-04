build_arg_builder!(
    NoEditArgBuilder,
    noedit,
    "no-edit",
    |builder: ArgBuilder<'a>| builder
        .with_short("n")
        .with_long("no-edit")
        .with_helptext("Do not edit")
        .with_takes_value(false)
        .with_required(false)
);
