build_arg_builder!(
    SelectArgBuilder,
    select,
    "select",
    |builder: ArgBuilder<'a>| builder
        .with_short("s")
        .with_long("select")
        .with_helptext("Select element")
        .with_takes_value(true)
        .with_required(false)
        .with_value_name("SELECT")
);
