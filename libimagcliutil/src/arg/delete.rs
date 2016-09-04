build_arg_builder!(
    DeleteArgBuilder,
    delete,
    "delete",
    |builder: ArgBuilder<'a>| builder
        .with_short("d")
        .with_long("delete")
        .with_takes_value(false)
        .with_required(false)
);
