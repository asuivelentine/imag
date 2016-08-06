build_arg_builder!(
    NameArgBuilder,
    name,
    "name",
    |builder: ArgBuilder<'a>| builder
        .with_short("n")
        .with_long("name")
        .with_helptext("select element by name")
        .with_takes_value(true)
        .with_required(false)
        .with_value_name("NAME")
);
