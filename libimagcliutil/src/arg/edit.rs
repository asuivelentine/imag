build_arg_builder!(
    EditArgBuilder,
    edit,
    "edit",
    |builder: ArgBuilder<'a>| builder
        .with_short("e")
        .with_long("edit")
        .with_takes_value(true)
        .with_required(false)
);
