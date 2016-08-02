build_arg_builder!(
    DateRangeArgBuilder,
    daterange,
    "date-range",
    |builder: ArgBuilder<'a>| builder
        .with_short("d")
        .with_long("date-range")
        .with_helptext("Select elements within the range")
        .with_takes_value(true)
        .with_required(false)
        .with_value_name("DATERANGE")
);
