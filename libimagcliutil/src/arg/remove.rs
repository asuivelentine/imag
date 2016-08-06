build_arg_builder!(
    RemoveArgBuilder,
    remove,
    "remove",
    |builder: ArgBuilder<'a>| builder
        .with_short("r")
        .with_long("remove")
        .with_helptext("remove the Element")
        .with_takes_value(true)
        .with_required(false)
        .with_value_name("REMOVE")
);
