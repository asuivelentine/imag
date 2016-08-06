build_arg_builder!(
    DeleteArgBuilder,
    delete,
    "delete",
    |builder: ArgBuilder<'a>| builder
        .with_short("d")
        .with_long("delete")
        .with_helptext("deletes the given element")
        .with_takes_value(false)
        .with_required(false)
        .without_value_name()
);
