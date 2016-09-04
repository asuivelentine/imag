build_arg_builder!(
    RangeArgBuilder,
    range,
    "range",
    |builder: ArgBuilder<'a>| builder
        .with_short("r")
        .with_long("range")
        .with_helptext("Specify range")
        .with_takes_value(true)
        .with_required(false)
        .with_value_name("RANGE")
);
