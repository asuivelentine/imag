build_arg_builder!(
    AddArgBuilder,
    add,
    "add",
    |builder: ArgBuilder<'a>| builder
        .with_short("a")
        .with_long("add")
        .with_takes_value(true)
        .with_required(false)
);
