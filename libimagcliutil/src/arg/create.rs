build_arg_builder!(
    CreateArgBuilder,
    create,
    "create",
    |builder: ArgBuilder<'a>| builder
        .with_short("c")
        .with_long("create")
        .with_takes_value(true)
        .with_required(false)
);
