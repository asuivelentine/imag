build_arg_builder!(
    AddArgBuilder,
    add,
    "add",
    |builder: ArgBuilder<'a>| builder
        .with_short("a")
        .with_long("add")
        .with_helptext("Add entry")
        .with_takes_value(true)
        .with_required(false)
        .with_value_name("NAME")
);
